use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use log::debug;
use minreq;
use pbr::{ProgressBar, Units};

pub fn download_to(url: &str, save_path: &str) {
    let save_path = Path::new(save_path);

    let resp = minreq::get(url)
        .send_lazy()
        .expect(&format!("请求 {} 失败", url));
    if resp.status_code != 200 {
        panic!("请求 {} 失败，状态码 {}", url, resp.status_code);
    }
    let data_length: usize = resp.headers["content-length"].parse().unwrap();
    let mut bar = ProgressBar::new(data_length as u64);
    bar.set_units(Units::Bytes);
    bar.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));

    let mut buf = Vec::with_capacity(data_length);

    for (i, unit) in resp.enumerate() {
        let (byte, _) = unit.unwrap();
        buf.push(byte);

        if i & 0b11111111 == 0b11111111 {
            bar.add(0b11111111);
        }
    }
    bar.finish_println("下载完成\n");

    let save_path = if save_path.is_dir() {
        save_path.join(Path::new(url).file_name().unwrap())
    } else {
        save_path.to_owned()
    };

    debug!("下载 {} 保存到 {}", url, save_path.display());

    let mut f = File::create(&save_path).unwrap();
    f.write(buf.as_slice()).unwrap();
}

pub fn ensure_dir<P: AsRef<Path>>(dir_path: P) {
    let dir_path = dir_path.as_ref();
    if !dir_path.exists() {
        fs::create_dir(dir_path).expect("自动创建目录失败");
    }
    if !dir_path.is_dir() {
        panic!("当前目录下存在非目录 {:?}", dir_path);
    }
}

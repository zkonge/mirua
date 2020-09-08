use std::{fs, io::Cursor, path::Path, process::Command};

use glob;
use log::{debug, info};
use minreq;
use pbr::{ProgressBar, Units};

#[cfg(unix)]
use flate2::read::GzDecoder;
#[cfg(unix)]
use tar::Archive;

#[cfg(windows)]
use std::io;
#[cfg(windows)]
use zip::ZipArchive;

const BASE_JRE_URL: &str = "https://mirrors.tuna.tsinghua.edu.cn/AdoptOpenJDK/11/jre";

macro_rules! jre_format {
    ($arch:expr, $os:expr, $suffex:expr) => {
        format!(
            "{}/{}/{}/OpenJDK11U-jre_{}_{}_openj9_11.0.8_10_openj9-0.21.0{}",
            BASE_JRE_URL, $arch, $os, $arch, $os, $suffex
        )
    };
}

pub fn check_jre(jre_path: &str) -> bool {
    debug!("jre_path: {}", jre_path);
    let jre_path = Path::new(jre_path);
    if !jre_path.exists() && !jre_path.with_extension("exe").exists() {
        info!("找不到 jar_path 指定的 java，开始下载 adoptopenjdk_openj9 到当前目录");
        info!("openj9相比于hotspot有更大的内存优势");
        return false;
    }
    match Command::new(jre_path).arg("-version").output() {
        Ok(output) => info!("{}", String::from_utf8_lossy(&output.stderr)),
        Err(e) => panic!(
            "jre_path 指定的 java 损坏，请考虑重新下载。错误：{}",
            e.to_string()
        ),
    }
    true
}

pub fn get_jre(jre_path: &str, jre_arch: Option<&str>) {
    let jre_path = Path::new(jre_path);

    let arch = if let Some(arch) = jre_arch {
        arch
    } else if cfg!(target_arch = "x86") {
        "x32"
    } else if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        panic!("这啥架构啊")
    };
    debug!("Arch: {}", arch);

    let (os, suffex) = if cfg!(target_os = "windows") {
        ("windows", ".zip")
    } else if cfg!(target_os = "linux") {
        ("linux", ".tar.gz")
    } else if cfg!(target_os = "macos") {
        panic!("暂不提供 MacOS 的 java 下载功能，修改当前目录下的 mirua.toml 手动指定 java 路径");
    } else {
        panic!("这啥系统啊");
    };
    debug!("OS: {}", os);

    //TODO:解析可下载的jre版本，tuna那个filelist不一定会更新
    let url = jre_format!(arch, os, suffex);

    info!("开始从 {} 下载 jre", url);

    let resp = minreq::get(url).send_lazy().expect("下载 jre 时出现问题");

    let jre_size: usize = resp.headers["content-length"].parse().unwrap();
    let mut jre_data = Vec::with_capacity(jre_size);

    let mut bar = ProgressBar::new(jre_size as u64);
    bar.set_units(Units::Bytes);
    bar.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));

    for (i, unit) in resp.enumerate() {
        let (byte, _) = unit.unwrap();
        jre_data.push(byte);

        //进度条会对接收效率产生相当大的影响，缓冲一下
        if i & 0b11111111 == 0b11111111 {
            bar.add(0b11111111);
        }
    }

    bar.finish_println("jre 下载完成");

    info!("提取 jre...");

    //adoptopenjdk压缩包顶层有一个目录，先解压，再重命名
    let jre_data = Cursor::new(jre_data);

    #[cfg(unix)]
    {
        let jre_data = GzDecoder::new(jre_data);
        let mut jre_data = Archive::new(jre_data);

        jre_data.unpack(".").unwrap();
    }

    #[cfg(windows)]
    {
        let mut archive = ZipArchive::new(jre_data).unwrap();
        let mut bar = ProgressBar::new(archive.len() as u64);
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            #[allow(deprecated)] //zip-rs给的example就是这么干的
            let outpath = file.sanitized_name();

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
            bar.inc();
        }

        bar.finish_println("提取完成");
    }

    //重命名
    //阴间写法，但确实能用
    let files_in_temp_dir = glob::glob("jdk-*-jre")
        .unwrap()
        .into_iter()
        .next()
        .expect("我那么大一个jre目录呢？")
        .unwrap();

    fs::rename(files_in_temp_dir, jre_path).unwrap();
}

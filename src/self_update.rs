use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;

use log::{debug, info, warn};
use minreq;
use serde::Deserialize;

use crate::MIRUA_VERSION;

#[derive(Debug, Deserialize)]
struct RepoSchema {
    tags: Vec<String>,
    versions: Vec<String>,
}

pub fn self_update() {
    info!("自动更新检测开始");
    let resp = minreq::get("https://data.jsdelivr.com/v1/package/gh/zkonge/mirua").send();
    if resp.is_err() || resp.as_ref().unwrap().status_code != 200 {
        warn!("连接更新服务器失败");
        return;
    }
    let resp = resp.unwrap();
    let result = resp.json::<RepoSchema>().unwrap();
    debug!("{:?}", result);

    //TODO:解析version，避免0.1.2>0.1.11的情况（版本暂时爬不到那么高）
    let latest_version = result.versions.iter().max().unwrap();

    info!("最新版本 v{}", latest_version);
    info!("本地版本 v{}", MIRUA_VERSION);

    if latest_version.as_str() <= MIRUA_VERSION {
        info!("不需要更新");
        return;
    }

    let resp = minreq::get(&format!(
        "https://cdn.jsdelivr.net/gh/zkonge/mirua-update/v{}/mirua_{}_{}",
        latest_version,
        env::consts::OS,
        env::consts::ARCH
    ))
    .send();

    if resp.is_err() || resp.as_ref().unwrap().status_code != 200 {
        warn!("下载更新失败");
        return;
    }
    let resp = resp.unwrap();
    let result = resp.into_bytes();

    let self_path = env::args().next().unwrap();
    let self_path = Path::new(&self_path);
    fs::rename(self_path, self_path.with_extension("miruaold")).expect("改名失败");

    let mut f = File::create(self_path).unwrap();
    f.write(&result).unwrap();

    info!("更新完成，重新运行软件即可体验新版");

    process::exit(0);
}

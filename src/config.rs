use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
    process,
};

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mirai {
    pub full: HashMap<String, String>,
    pub maven: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JRE {
    pub path: Option<String>,
    pub arch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "self-update")]
    pub self_update: bool,
    #[serde(rename = "bootstrap-commands")]
    pub bootstrap_commands: Vec<String>,
    pub jre: JRE,
    pub entrypoint: String,
    pub mirai: Mirai,
}

impl Config {
    //获取全局配置文件，不存在则创建
    pub fn get_config(config_path: &str) -> Config {
        let config_path = Path::new(config_path);

        let buf = if config_path.exists() && config_path.is_file() {
            let mut buf = String::new();

            let mut f = File::open(config_path).expect("打开文件失败");
            f.read_to_string(&mut buf).expect("打开文件失败");

            buf
        } else {
            let buf = include_str!("mirua.toml.template").to_owned();

            let mut f = File::create(config_path).expect("写入文件失败");
            f.write(buf.as_bytes()).expect("写入文件失败");

            info!("默认配置文件已经在当前目录生成，请确认后再次运行本程序");
            process::exit(0);
        };

        toml::from_str::<Self>(&buf).expect("解析配置文件失败")
    }
}

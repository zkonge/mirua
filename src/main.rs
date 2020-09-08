use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use glob;
use log::{debug, info, warn};
use minreq;
use pbr::ProgressBar;
use simple_logger::SimpleLogger;
use threadpool::ThreadPool;

mod config;
use config::Config;

mod jre;
mod pom;
mod utils;

const MIRAI_PATH: &str = "./content";
const JRE_PATH: &str = "./runtime";
const JAVA_PATH: &str = "./runtime/bin/java";
const MIRUA_VERSION: &str = env!("CARGO_PKG_VERSION");

fn download_maven(pom: &str, save_path: &'static str) {
    //TODO:save_path换path泛型
    let url = pom;
    let pool = ThreadPool::new(12);
    info!("依照 {} 拉取依赖", url);
    let dependencies = pom::get_dependencies(url);
    info!("需要下载依赖数量：{}", dependencies.len());

    let pb = Arc::new(Mutex::new(ProgressBar::new(dependencies.len() as u64)));
    for dependency in dependencies {
        let url = dependency.to_jar_url();
        let pb = pb.clone();
        pool.execute(move || {
            let content = minreq::get(&url).send().unwrap();
            if content.status_code != 200 {
                return;
            }
            let path = Path::new(&url).file_name().unwrap();
            let mut file = File::create(Path::new(save_path).join(path)).unwrap();
            file.write(content.as_bytes()).unwrap();
            pb.lock().unwrap().inc();
        });
    }
    pool.join();
    pb.lock().unwrap().finish_println("依赖下载完成\n");
}

//未完成
fn self_update() {
    info!("自动更新检测开始");
    info!("然而更新服务还没上线");
}

fn parse_mirai_from_config(project: &str, version: &str) -> (String, String, String) {
    let r: Vec<_> = project.rsplitn(2, ':').collect();
    let mut r = r.into_iter().rev();
    let (group_id, artifact_id) = (r.next().unwrap(), r.next().unwrap());
    (
        group_id.to_owned(),
        artifact_id.to_owned(),
        version.to_owned(),
    )
}

fn init_log() {
    let level = match std::env::var("RUST_LOG") {
        Ok(x) => match x.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            _ => log::LevelFilter::Error,
        },
        _ => log::LevelFilter::Info,
    };
    SimpleLogger::new().with_level(level).init().unwrap();
}

fn main() {
    init_log();

    info!("Mirua v{}", MIRUA_VERSION);

    let config = Config::get_config("mirua.toml");
    debug!("{:?}", config);

    //自动更新时会将旧文件重命名为 "<原文件名>.miruaold"
    //新版本启动时删除
    glob::glob("*.miruaold")
        .expect("读取当前目录失败")
        .for_each(|p| {
            let p = p.expect("读取当前目录失败");
            match fs::remove_file(p) {
                Ok(()) => info!("检测到旧版本文件并删除"),
                Err(e) => warn!("删除旧版本文件失败, {}", e.to_string()),
            };
        });

    if config.self_update {
        self_update();
    }

    utils::ensure_dir(MIRAI_PATH);

    let java_path = config.jre.path.as_deref().unwrap_or(JAVA_PATH);
    if !jre::check_jre(java_path) {
        jre::get_jre(JRE_PATH, config.jre.arch.as_deref());
    }

    //检查maven依赖的jar
    for (project, version) in config.mirai.maven.iter() {
        let (group_id, artifact_id, version) = parse_mirai_from_config(project, version);
        let jar_url = pom::build_maven_jar_url(&group_id, &artifact_id, &version);
        let pom_url = pom::build_maven_pom_url(&group_id, &artifact_id, &version);
        let jar_name = Path::new(&jar_url).file_name().unwrap().to_str().unwrap();
        let jar_path = Path::new(MIRAI_PATH).join(jar_name);
        if !jar_path.exists() {
            info!("缺少 {}，开始下载", jar_name);
            download_maven(&pom_url, MIRAI_PATH);
        }
    }

    //检查全打包jar
    for (project, version) in config.mirai.full.iter() {
        let (group_id, artifact_id, version) = parse_mirai_from_config(project, version);
        let jar_url =
            pom::build_maven_jar_url(&group_id, &artifact_id, &version).replace(".jar", "-all.jar");
        let jar_name = Path::new(&jar_url).file_name().unwrap().to_str().unwrap();
        debug!("jar_name:{}", jar_name);
        let jar_path = Path::new(MIRAI_PATH).join(jar_name);
        debug!("jar_path:{}", jar_path.display());
        if !jar_path.exists() {
            info!("缺少 {}，开始下载", jar_name);
            utils::download_to(&jar_url, MIRAI_PATH);
        }
    }

    let mut child = Command::new(java_path)
        .args(&["-cp", Path::new(MIRAI_PATH).join("*").to_str().unwrap()])
        //.arg("-Dorg.jline.terminal.dumb=true")
        //.arg("-Djansi.passthrough=true")
        .arg(config.entrypoint)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let bootstrap_commands = {
        let mut t = config.bootstrap_commands.join("\n");
        if !t.is_empty() {
            t.push('\n');
        }
        t
    };

    if !bootstrap_commands.is_empty() {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write(bootstrap_commands.as_bytes()).unwrap();
        child_stdin.flush().unwrap();
    }

    match child.wait() {
        Ok(code) => info!("mirai退出，状态码 {}", code),
        Err(e) => {
            log::error!("子进程异常 {}", e.to_string());
            child.kill().unwrap();
        }
    }
}

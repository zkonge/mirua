use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::thread;

use log::debug;
use minreq;
use quick_xml::de;

pub mod structure;
use structure::{Dependency, Exclusion, Project};

const MAVEN_URL: &str = "https://maven.aliyun.com/repository/public";

fn build_maven_base_url(group_id: &str, artifact_id: &str, version: &str) -> String {
    let prefix = group_id.split('.').collect::<Vec<_>>().join("/");
    format!(
        "{}/{}/{}/{}/{}-{}",
        MAVEN_URL, prefix, artifact_id, version, artifact_id, version
    )
}

pub fn build_maven_pom_url(group_id: &str, artifact_id: &str, version: &str) -> String {
    format!(
        "{}.pom",
        build_maven_base_url(group_id, artifact_id, version)
    )
}

pub fn build_maven_jar_url(group_id: &str, artifact_id: &str, version: &str) -> String {
    format!(
        "{}.jar",
        build_maven_base_url(group_id, artifact_id, version)
    )
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DependencyInfo {
    pub group_id: String,
    pub artifact_id: String,
}

impl DependencyInfo {
    fn new(group_id: String, artifact_id: String) -> Self {
        Self {
            group_id,
            artifact_id,
        }
    }
}

pub fn get_dependencies(url: &str) -> HashSet<Dependency> {
    let result: HashMap<DependencyInfo, Dependency> = HashMap::new();
    let result = Arc::new(RwLock::new(result));
    _get_dependencies(url, HashSet::new(), result.clone());
    Arc::try_unwrap(result)
        .unwrap()
        .into_inner()
        .unwrap()
        .into_iter()
        .map(|x| x.1)
        .collect()
}

fn _get_dependencies(
    url: &str,
    exclusions: HashSet<Exclusion>,
    result: Arc<RwLock<HashMap<DependencyInfo, Dependency>>>,
) {
    debug!("获取 {}", url);
    let resp = minreq::get(url)
        .send()
        .expect(&format!("获取 {} 失败", url))
        .as_str()
        .expect(&format!("获取 {} 失败", url))
        .to_owned();
    let project: Project = de::from_str(&resp).expect(&format!("解析 {} 失败", url));

    //获取当前project信息，生成Dependency

    let (group_id, artifact_id, version) = match &project.parent {
        Some(parent) => (
            project
                .group_id
                .as_ref()
                .unwrap_or_else(|| &parent.group_id),
            &project.artifact_id,
            project
                .version
                .as_ref()
                .unwrap_or_else(|| parent.version.as_ref().unwrap()),
        ),
        None => (
            project.group_id.as_ref().unwrap(),
            &project.artifact_id,
            project.version.as_ref().unwrap(),
        ),
    };

    result.write().unwrap().insert(
        DependencyInfo::new(group_id.to_owned(), artifact_id.to_owned()),
        Dependency {
            group_id: group_id.to_owned(),
            artifact_id: artifact_id.to_owned(),
            version: Some(version.to_owned()),
            ..Default::default()
        },
    );

    //有parent，parent成为依赖，并抓取parent的子依赖
    if let Some(parent) = &project.parent {
        _get_dependencies(&parent.to_pom_url(), HashSet::new(), result.clone());
    }

    //没子依赖了
    if project.dependencies.is_none() {
        return;
    }

    //分析接下来需要拉取的依赖
    let dependencies: Vec<Dependency> = project
        .dependencies
        .unwrap()
        .dependencies
        .into_iter()
        .filter(|x| {
            let exclusion = Exclusion {
                group_id: x.group_id.to_owned(),
                artifact_id: x.artifact_id.to_owned(),
            };
            //排除已经抓取过的
            if result.read().unwrap().contains_key(&DependencyInfo::new(
                x.group_id.to_owned(),
                x.artifact_id.to_owned(),
            )) {
                return false;
            }

            //排除指明的exclusion
            if exclusions.contains(&exclusion) {
                return false;
            }

            //默认scope为compile
            if let Some(scope) = x.scope.as_deref() {
                //排除测试用的依赖
                if scope == "test" {
                    return false;
                }
            }

            //不需要optional的依赖
            if let Some(optional) = x.optional {
                if optional == true {
                    return false;
                }
            }

            true
        })
        .collect();

    //拉依赖
    dependencies
        .into_iter()
        .map(|dependency| {
            let url = dependency.to_pom_url();
            let mut candidated_exclusions: HashSet<Exclusion> = HashSet::new();
            if let Some(x) = dependency.exclusions {
                x.exclusions.into_iter().for_each(|x| {
                    candidated_exclusions.insert(x);
                })
            }
            let rc = result.clone();
            thread::spawn(move || {
                _get_dependencies(&url.to_owned(), candidated_exclusions, rc);
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|x| x.join().unwrap_or(()));
}

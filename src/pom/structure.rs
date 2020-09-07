// The MIT License (MIT)
//
// Copyright (C) 2020 Fabio Valentini(https://pagure.io/ironthree/pommes)
// Copyright (C) 2020 zkonge
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![allow(dead_code)]
use serde::Deserialize;

use crate::pom::{build_maven_jar_url, build_maven_pom_url};

macro_rules! WithProjectURL {
    ($T:ty) => {
        impl $T {
            pub fn to_pom_url(&self) -> String {
                build_maven_pom_url(
                    &self.group_id,
                    &self.artifact_id,
                    self.version.as_deref().unwrap(),
                )
            }
            pub fn to_jar_url(&self) -> String {
                build_maven_jar_url(
                    &self.group_id,
                    &self.artifact_id,
                    self.version.as_deref().unwrap(),
                )
            }
        }
    };
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Project {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub build: Option<Build>,
    pub contributors: Option<Contributors>,
    pub dependencies: Option<Dependencies>,
    #[serde(rename = "dependencyManagement")]
    pub dependency_management: Option<Dependencies>,
    pub description: Option<String>,
    pub developers: Option<Developers>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "inceptionYear")]
    pub inception_year: Option<String>,
    pub licenses: Option<Licenses>,
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    pub modules: Option<Modules>,
    pub name: Option<String>,
    pub organization: Option<Organization>,
    pub packaging: Option<String>,
    pub parent: Option<Parent>,
    #[serde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<PluginRepositories>,
    pub profiles: Option<Profiles>,
    pub repositories: Option<Repositories>,
    pub url: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Licenses {
    #[serde(rename = "license", default)]
    pub licenses: Vec<License>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct License {
    pub comments: Option<String>,
    pub distribution: Option<String>,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Organization {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Developers {
    #[serde(rename = "developer", default)]
    pub developers: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Contributors {
    #[serde(rename = "contributor", default)]
    pub contributors: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Person {
    pub email: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "organizationUrl")]
    pub organization_url: Option<String>,
    pub roles: Option<Roles>,
    pub timezone: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Roles {
    #[serde(rename = "role", default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Parent {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub version: Option<String>,
}
WithProjectURL!(Parent);

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Dependencies {
    #[serde(rename = "dependency", default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Dependency {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "type")]
    pub dtype: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub optional: Option<bool>,
    pub scope: Option<String>,
    pub version: Option<String>,
    pub exclusions: Option<Exclusions>,
}
WithProjectURL!(Dependency);

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Exclusions {
    #[serde(rename = "exclusion", default)]
    pub exclusions: Vec<Exclusion>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Exclusion {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Modules {
    #[serde(rename = "module", default)]
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Build {
    #[serde(rename = "defaultGoal")]
    pub default_goal: Option<String>,
    pub directory: Option<String>,
    pub extensions: Option<Extensions>,
    pub filters: Option<Filters>,
    #[serde(rename = "finalName")]
    pub final_name: Option<String>,
    #[serde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,
    pub plugins: Option<Plugins>,
    pub resources: Option<Resources>,
    #[serde(rename = "testResources")]
    pub test_resources: Option<TestResources>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Filters {
    #[serde(rename = "filter", default)]
    pub filters: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Resources {
    #[serde(rename = "resource", default)]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct TestResources {
    #[serde(rename = "testResource", default)]
    pub test_resources: Vec<Resource>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Resource {
    pub directory: String,
    pub excludes: Option<Excludes>,
    pub filtering: Option<bool>,
    pub includes: Option<Includes>,
    #[serde(rename = "targetPath")]
    pub target_path: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Includes {
    #[serde(rename = "include", default)]
    pub includes: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Excludes {
    #[serde(rename = "exclude", default)]
    pub excludes: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Plugins {
    #[serde(rename = "plugin", default)]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct PluginManagement {
    pub plugins: Plugins,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Plugin {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub configuration: Option<Configuration>,
    pub dependencies: Option<Dependencies>,
    pub executions: Option<Executions>,
    pub extensions: Option<bool>,
    #[serde(rename = "groupId", default = "default_plugin_group_id")]
    pub group_id: String,
    pub inherited: Option<bool>,
    pub version: Option<String>,
}

fn default_plugin_group_id() -> String {
    String::from("org.apache.maven.plugins")
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Executions {
    #[serde(rename = "execution", default)]
    pub executions: Vec<Execution>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Execution {
    pub configuration: Option<Configuration>,
    pub goals: Option<Goals>,
    pub id: Option<String>,
    pub inherited: Option<bool>,
    pub phase: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Configuration {
    // empty because this is different for every plugin and execution :(
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Goals {
    #[serde(rename = "goal", default)]
    pub goals: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Extensions {
    #[serde(rename = "extension", default)]
    pub extensions: Vec<Extension>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Extension {
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Profiles {
    #[serde(rename = "profile", default)]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Profile {
    pub activation: Option<Activation>,
    pub id: Option<String>,
    #[serde(rename = "dependencyManagement")]
    pub dependency_management: Option<DependencyManagement>,
    pub modules: Option<Modules>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Activation {
    #[serde(rename = "activeByDefault")]
    pub active_by_default: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct DependencyManagement {
    pub dependencies: Dependencies,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Repositories {
    #[serde(rename = "repository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct PluginRepositories {
    #[serde(rename = "pluginRepository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Repository {
    pub id: String,
    pub url: String,
}

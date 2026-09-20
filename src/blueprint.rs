use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmberBlueprint {
    pub package: PackageMeta,
    pub build: BuildMeta,
    pub rules: RulesMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildMeta {
    pub artifact: PathBuf,
    pub system_deps: Vec<String>,
    pub env: HashMap<String, String>,
    pub prebuild: Option<Vec<String>>,
    pub buildcmd: Vec<String>,
    pub postbuild: Option<Vec<String>>,
    pub purge_paths: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RulesMeta {
    pub min_ember_version: String,
    pub strict_deps: bool,
}

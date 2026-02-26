use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::core::domain::skill::SkillName;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    pub name: SkillName,
    pub description: String,
    #[serde(rename = "install-dir", skip_serializing_if = "Option::is_none")]
    pub install_dir: Option<PathBuf>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

impl Lockfile {
    pub fn new(name: SkillName, description: String, install_dir: Option<PathBuf>) -> Self {
        Self {
            name,
            description,
            install_dir,
            dependencies: HashMap::new(),
        }
    }

    pub fn set_commit(&mut self, key: String, commit_hash: String) {
        self.dependencies.insert(key, commit_hash);
    }

    pub fn remove_commit(&mut self, key: &str) {
        self.dependencies.remove(key);
    }

    pub fn get_commit(&self, key: &str) -> Option<&String> {
        self.dependencies.get(key)
    }
}

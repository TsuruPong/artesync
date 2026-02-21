use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::core::domain::skill::{SkillName, SkillSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub name: SkillName,
    pub description: String,
    #[serde(rename = "install-dir", skip_serializing_if = "Option::is_none")]
    pub install_dir: Option<PathBuf>,
    #[serde(default)]
    pub dependencies: HashMap<String, SkillSource>,
}

impl Manifest {
    pub fn new(name: SkillName, description: String) -> Self {
        Self {
            name,
            description,
            install_dir: None,
            dependencies: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, key: String, source: SkillSource) {
        self.dependencies.insert(key, source);
    }

    pub fn remove_dependency(&mut self, key: &str) {
        self.dependencies.remove(key);
    }
}

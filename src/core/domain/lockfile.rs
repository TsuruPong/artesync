use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self {
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

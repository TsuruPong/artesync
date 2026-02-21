use serde::{Deserialize, Serialize};
use crate::core::domain::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillName(String);

impl SkillName {
    pub fn new(name: &str) -> Result<Self, AppError> {
        if name.is_empty() || name.len() > 64 {
            return Err(AppError::Skill(format!("Skill name length must be between 1 and 64 characters: {}", name)));
        }
        if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(AppError::Skill(format!("Skill name must contain only lowercase alphanumeric characters and hyphens: {}", name)));
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SkillName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillSource(String);

impl SkillSource {
    pub fn new(source: &str) -> Self {
        Self(source.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

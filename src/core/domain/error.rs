use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("manifest error: {0}")]
    Manifest(String),
    #[error("skill error: {0}")]
    Skill(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("system error: {0}")]
    System(String),
}

use std::path::Path;
use std::fs;
use crate::core::domain::lockfile::Lockfile;
use crate::core::domain::error::AppError;
use crate::core::port::lockfile_repository::LockfileRepository;

pub struct FileLockfileRepository;

impl FileLockfileRepository {
    pub fn new() -> Self {
        Self
    }
}

impl LockfileRepository for FileLockfileRepository {
    fn load(&self, path: &Path) -> Result<Lockfile, AppError> {
        if !path.exists() {
            return Ok(Lockfile::new());
        }

        let content = fs::read_to_string(path).map_err(AppError::Io)?;
        let lockfile: Lockfile = serde_json::from_str(&content)
            .map_err(|e| AppError::System(format!("Failed to parse lockfile: {}", e)))?;

        Ok(lockfile)
    }

    fn save(&self, path: &Path, lockfile: &Lockfile) -> Result<(), AppError> {
        let content = serde_json::to_string_pretty(lockfile)
            .map_err(|e| AppError::System(format!("Failed to serialize lockfile: {}", e)))?;
        fs::write(path, content).map_err(AppError::Io)?;
        Ok(())
    }
}

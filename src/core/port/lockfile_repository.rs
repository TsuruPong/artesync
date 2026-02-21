use std::path::Path;
use crate::core::domain::lockfile::Lockfile;
use crate::core::domain::error::AppError;

pub trait LockfileRepository {
    fn load(&self, path: &Path) -> Result<Lockfile, AppError>;
    fn save(&self, path: &Path, lockfile: &Lockfile) -> Result<(), AppError>;
}

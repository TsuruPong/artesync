use std::path::Path;
use crate::core::domain::error::AppError;

pub trait FileSystem {
    fn exists(&self, path: &Path) -> bool;
    fn remove_dir_all(&self, path: &Path) -> Result<(), AppError>;
    fn move_dir(&self, src: &Path, dest: &Path) -> Result<(), AppError>;
}

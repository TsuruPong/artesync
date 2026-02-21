use std::fs;
use std::path::Path;
use fs_extra::dir::{copy, CopyOptions};
use crate::core::port::file_system::FileSystem;
use crate::core::domain::error::AppError;

pub struct LocalFileSystem;

impl LocalFileSystem {
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for LocalFileSystem {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), AppError> {
        if path.exists() {
            fs::remove_dir_all(path).map_err(AppError::Io)?;
        }
        Ok(())
    }

    fn move_dir(&self, src: &Path, dest: &Path) -> Result<(), AppError> {
        if dest.exists() {
            fs::remove_dir_all(dest).map_err(AppError::Io)?;
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(AppError::Io)?;
        }
        
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        
        copy(src, dest, &options).map_err(|e| AppError::System(format!("Failed to copy directory: {}", e)))?;
        fs::remove_dir_all(src).map_err(AppError::Io)?;
        
        Ok(())
    }
}

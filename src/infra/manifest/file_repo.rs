use std::fs;
use std::path::Path;
use crate::core::domain::manifest::Manifest;
use crate::core::domain::error::AppError;
use crate::core::port::manifest_repository::ManifestRepository;

pub struct FileManifestRepository;

impl FileManifestRepository {
    pub fn new() -> Self {
        Self
    }
}

impl ManifestRepository for FileManifestRepository {
    fn load(&self, path: &Path) -> Result<Manifest, AppError> {
        let content = fs::read_to_string(path).map_err(AppError::Io)?;
        let manifest: Manifest = serde_json::from_str(&content).map_err(AppError::Serialization)?;
        Ok(manifest)
    }

    fn save(&self, path: &Path, manifest: &Manifest) -> Result<(), AppError> {
        let content = serde_json::to_string_pretty(manifest).map_err(AppError::Serialization)?;
        fs::write(path, content).map_err(AppError::Io)?;
        Ok(())
    }
}

use std::path::Path;
use crate::core::domain::manifest::Manifest;
use crate::core::domain::error::AppError;

pub trait ManifestRepository {
    fn load(&self, path: &Path) -> Result<Manifest, AppError>;
    fn save(&self, path: &Path, manifest: &Manifest) -> Result<(), AppError>;
}

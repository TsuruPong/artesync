use std::path::Path;
use crate::core::domain::error::AppError;
use crate::core::port::manifest_repository::ManifestRepository;

pub struct ListUseCase<'a, M: ManifestRepository> {
    manifest_repo: &'a M,
}

impl<'a, M: ManifestRepository> ListUseCase<'a, M> {
    pub fn new(manifest_repo: &'a M) -> Self {
        Self { manifest_repo }
    }

    pub fn execute(&self, dir: &Path) -> Result<Vec<(String, String)>, AppError> {
        let manifest_path = dir.join("skills.arsync");
        let manifest = self.manifest_repo.load(&manifest_path)?;

        let mut skills = Vec::new();
        for (key, source) in &manifest.dependencies {
            skills.push((key.clone(), source.as_str().to_string()));
        }

        Ok(skills)
    }
}

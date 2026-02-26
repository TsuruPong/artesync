use std::path::Path;
use colored::Colorize;
use crate::core::domain::error::AppError;
use crate::core::port::manifest_repository::ManifestRepository;
use crate::core::port::file_system::FileSystem;
use crate::core::port::lockfile_repository::LockfileRepository;

pub struct UninstallUseCase<'a, M: ManifestRepository, F: FileSystem, L: LockfileRepository> {
    manifest_repo: &'a M,
    file_system: &'a F,
    lockfile_repo: &'a L,
}

impl<'a, M: ManifestRepository, F: FileSystem, L: LockfileRepository> UninstallUseCase<'a, M, F, L> {
    pub fn new(manifest_repo: &'a M, file_system: &'a F, lockfile_repo: &'a L) -> Self {
        Self { manifest_repo, file_system, lockfile_repo }
    }

    pub fn execute(&self, dir: &Path, skill_key: &str) -> Result<(), AppError> {
        let manifest_path = dir.join("skills.arsync");
        let mut manifest = self.manifest_repo.load(&manifest_path)?;

        let lockfile_path = dir.join("skills-lock.arsync");
        let mut lockfile = self.lockfile_repo.load(&lockfile_path).unwrap_or_else(|_| {
            crate::core::domain::lockfile::Lockfile::new(
                manifest.name.clone(),
                manifest.description.clone(),
                manifest.install_dir.clone()
            )
        });

        if !manifest.dependencies.contains_key(skill_key) {
            return Err(AppError::System(format!("Skill '{}' not found in manifest", skill_key)));
        }

        println!("{} {}...", "=> Uninstalling".cyan().bold(), skill_key);

        let install_base = manifest.install_dir.clone().unwrap_or_else(|| dir.to_path_buf());
        let dest_path = install_base.join(skill_key);

        if self.file_system.exists(&dest_path) {
            self.file_system.remove_dir_all(&dest_path)?;
        }

        manifest.remove_dependency(skill_key);
        self.manifest_repo.save(&manifest_path, &manifest)?;

        lockfile.remove_commit(skill_key);
        let _ = self.lockfile_repo.save(&lockfile_path, &lockfile);

        println!("{} Successfully uninstalled {}", "✔".green().bold(), skill_key);

        Ok(())
    }
}

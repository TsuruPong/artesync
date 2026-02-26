use std::path::Path;
use colored::Colorize;
use crate::core::domain::error::AppError;
use crate::core::port::manifest_repository::ManifestRepository;
use crate::core::port::skill_fetcher::SkillFetcher;
use crate::core::port::lockfile_repository::LockfileRepository;
use crate::core::domain::validation::validate_skill_soft;

pub struct UpdateUseCase<'a, M: ManifestRepository, S: SkillFetcher, L: LockfileRepository> {
    manifest_repo: &'a M,
    skill_fetcher: &'a S,
    lockfile_repo: &'a L,
}

impl<'a, M: ManifestRepository, S: SkillFetcher, L: LockfileRepository> UpdateUseCase<'a, M, S, L> {
    pub fn new(manifest_repo: &'a M, skill_fetcher: &'a S, lockfile_repo: &'a L) -> Self {
        Self { manifest_repo, skill_fetcher, lockfile_repo }
    }

    pub fn execute(&self, dir: &Path, skill_key_opt: Option<&str>) -> Result<(), AppError> {
        let manifest_path = dir.join("skills.arsync");
        let manifest = self.manifest_repo.load(&manifest_path)?;

        let lockfile_path = dir.join("skills-lock.arsync");
        let mut lockfile = self.lockfile_repo.load(&lockfile_path).unwrap_or_else(|_| crate::core::domain::lockfile::Lockfile::new());

        let install_base = manifest.install_dir.clone().unwrap_or_else(|| dir.to_path_buf());

        let keys_to_update: Vec<String> = if let Some(key) = skill_key_opt {
            if !manifest.dependencies.contains_key(key) {
                return Err(AppError::System(format!("Skill '{}' not found in manifest", key)));
            }
            vec![key.to_string()]
        } else {
            manifest.dependencies.keys().cloned().collect()
        };

        if keys_to_update.is_empty() {
            println!("{} Nothing to update.", "ℹ".blue().bold());
            return Ok(());
        }

        println!("{} Checking {} skills for updates...", "=>".cyan().bold(), keys_to_update.len());

        for key in keys_to_update {
            if let Some(source) = manifest.dependencies.get(&key) {
                println!("  {} {}...", "Checking".yellow(), key);
                
                let dest_path = install_base.join(&key);
                let current_hash = lockfile.get_commit(&key).cloned().unwrap_or_else(|| "unknown".to_string());
                
                // Cheaply resolve the remote hash via ls-remote before doing expensive fetch+copy
                let remote_hash = self.skill_fetcher.resolve_remote_hash(source)?;
                
                if current_hash == remote_hash && current_hash != "unknown" {
                    println!("  {} {} is already up to date.", "✔".green(), key);
                } else {
                    // Only fetch and overwrite files when the hash actually changed
                    let new_hash = self.skill_fetcher.fetch(source, &dest_path, None)?;
                    
                    // Run soft validation
                    validate_skill_soft(&dest_path, &key);
                    
                    println!("  {} {} updated ({} -> {}).", "✔".green(), key, &current_hash[..8.min(current_hash.len())], &new_hash[..8.min(new_hash.len())]);
                    lockfile.set_commit(key.clone(), new_hash);
                }
            }
        }

        let _ = self.lockfile_repo.save(&lockfile_path, &lockfile);
        println!("{} Update complete.", "✔".green().bold());

        Ok(())
    }
}

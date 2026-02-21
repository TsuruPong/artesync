use std::path::Path;
use colored::Colorize;
use crate::core::domain::skill::{SkillName, SkillSource};
use crate::core::domain::error::AppError;
use crate::core::port::manifest_repository::ManifestRepository;
use crate::core::port::skill_fetcher::SkillFetcher;
use crate::core::port::lockfile_repository::LockfileRepository;
use crate::core::domain::validation::validate_skill_soft;

pub struct InstallUseCase<'a, M: ManifestRepository, S: SkillFetcher, L: LockfileRepository> {
    manifest_repo: &'a M,
    skill_fetcher: &'a S,
    lockfile_repo: &'a L,
}

impl<'a, M: ManifestRepository, S: SkillFetcher, L: LockfileRepository> InstallUseCase<'a, M, S, L> {
    pub fn new(manifest_repo: &'a M, skill_fetcher: &'a S, lockfile_repo: &'a L) -> Self {
        Self { manifest_repo, skill_fetcher, lockfile_repo }
    }

    pub fn execute(&self, dir: &Path, source_opt: Option<String>) -> Result<(), AppError> {
        let manifest_path = dir.join("skills.arsync");
        let mut manifest = self.manifest_repo.load(&manifest_path)?;

        let lockfile_path = dir.join("skills.arsync.lock");
        let mut lockfile = self.lockfile_repo.load(&lockfile_path).unwrap_or_else(|_| crate::core::domain::lockfile::Lockfile::new());

        let install_base = manifest.install_dir.clone().unwrap_or_else(|| dir.to_path_buf());

        if let Some(source_str) = source_opt {
            let source = SkillSource::new(&source_str);
            
            let raw_name = extract_skill_name_raw(&source_str);
            // Validate through SkillName domain rules
            let skill_name = SkillName::new(&raw_name)?;
            let skill_key = skill_name.as_str().to_string();

            if let Some(existing_source) = manifest.dependencies.get(&skill_key) {
                if existing_source.as_str() != source_str {
                    return Err(AppError::System(format!(
                        "A different skill named '{}' is already installed from ({}).\nPlease uninstall it first before installing from {}.",
                        skill_key, existing_source.as_str(), source_str
                    )));
                }
            }
            
            println!("{} {}...", "=> Installing".cyan().bold(), source.as_str());

            let dest_path = install_base.join(&skill_key);
            let commit_hash = self.skill_fetcher.fetch(&source, &dest_path, None)?;

            // Run soft validation warnings
            validate_skill_soft(&dest_path, &skill_key);

            manifest.add_dependency(skill_key.clone(), source);
            self.manifest_repo.save(&manifest_path, &manifest)?;

            lockfile.set_commit(skill_key.clone(), commit_hash);
            let _ = self.lockfile_repo.save(&lockfile_path, &lockfile);

            println!("{} Successfully installed {} to '{}'", "✔".green().bold(), source_str, dest_path.display());
        } else {
            println!("{} Installing all dependencies from manifest...", "=>".cyan().bold());
            let mut count = 0;
            for (key, source) in &manifest.dependencies {
                println!("  {} {}...", "Fetching".yellow(), source.as_str());
                let dest_path = install_base.join(key);
                
                // If a hash is locked, we want to check that out specifically to guarantee identical environments across machines
                let target_commit = lockfile.get_commit(key).map(|s| s.as_str());
                
                let commit_hash = self.skill_fetcher.fetch(source, &dest_path, target_commit)?;
                
                // Run soft validation warnings
                validate_skill_soft(&dest_path, key);

                lockfile.set_commit(key.clone(), commit_hash); // Set it in case it wasn't there
                count += 1;
            }
            
            let _ = self.lockfile_repo.save(&lockfile_path, &lockfile);

            if count > 0 {
                println!("{} Installed {} skills", "✔".green().bold(), count);
            } else {
                println!("{} No dependencies found in manifest.", "ℹ".blue().bold());
            }
        }

        Ok(())
    }
}

/// Extracts the raw skill name string from a source specifier.
/// The returned name is then validated through `SkillName::new()` at the call site.
fn extract_skill_name_raw(source: &str) -> String {
    // First, strip off any branch or tag provided at the end (e.g. #branch or @tag)
    let path_str = if let Some(idx) = source.find('@') {
        &source[..idx]
    } else if let Some(idx) = source.find('#') {
        &source[..idx]
    } else {
        source
    };

    // Now path_str is just "owner/repo" or "owner/repo/path/to/folder"
    let parts: Vec<&str> = path_str.split('/').collect();
    
    if parts.len() > 2 {
        // Has subfolders, take the very last segment
        return parts.last().unwrap().to_string();
    }
    
    // Otherwise it's just "owner/repo"
    let repo_part = parts.get(1).unwrap_or(&path_str);
    repo_part.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_skill_name_raw() {
        // Basic owner/repo
        assert_eq!(extract_skill_name_raw("BurntSushi/toml"), "toml");
        assert_eq!(extract_skill_name_raw("owner/my-cool-repo"), "my-cool-repo");

        // With branch or tag
        assert_eq!(extract_skill_name_raw("owner/repo#main"), "repo");
        assert_eq!(extract_skill_name_raw("owner/repo@v1.0.0"), "repo");

        // With subfolders
        assert_eq!(extract_skill_name_raw("owner/repo/subfolder/skill"), "skill");
        assert_eq!(extract_skill_name_raw("owner/repo/deep/path/to/my-skill"), "my-skill");

        // With subfolders and branch/tag at the end
        assert_eq!(extract_skill_name_raw("owner/repo/subfolder/skill#branch"), "skill");
        assert_eq!(extract_skill_name_raw("owner/repo/subfolder/skill@tag"), "skill");
    }

    #[test]
    fn test_skill_name_validation_applies() {
        // Valid name passes through SkillName
        let raw = extract_skill_name_raw("owner/valid-skill");
        assert!(SkillName::new(&raw).is_ok());

        // Invalid name (uppercase) is rejected by SkillName
        let raw_invalid = extract_skill_name_raw("owner/InvalidSkill");
        assert!(SkillName::new(&raw_invalid).is_err());
    }
}

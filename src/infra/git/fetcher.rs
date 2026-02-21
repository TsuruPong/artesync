use std::path::Path;
use std::process::Command;
use fs_extra::dir::{copy, CopyOptions};
use crate::core::domain::skill::SkillSource;
use crate::core::domain::error::AppError;
use crate::core::port::skill_fetcher::SkillFetcher;

pub struct CliGitFetcher;

impl CliGitFetcher {
    pub fn new() -> Self {
        Self
    }
}

impl SkillFetcher for CliGitFetcher {
    fn fetch(&self, source: &SkillSource, dest: &Path, target_commit: Option<&str>) -> Result<String, AppError> {
        let mut source_str = source.as_str();
        let mut branch_or_tag = None;

        // Extract branch/tag first from the very end
        if let Some(idx) = source_str.find('@') {
            branch_or_tag = Some(&source_str[idx + 1..]);
            source_str = &source_str[..idx];
        } else if let Some(idx) = source_str.find('#') {
            branch_or_tag = Some(&source_str[idx + 1..]);
            source_str = &source_str[..idx];
        }

        // Now split the path
        let mut parts = source_str.splitn(3, '/');
        let owner = parts.next().ok_or_else(|| AppError::System("Invalid source: missing owner".to_string()))?;
        let repo = parts.next().ok_or_else(|| AppError::System("Invalid source: missing repo".to_string()))?;
        let subfolder = parts.next().unwrap_or("");

        let url = format!("https://github.com/{}/{}.git", owner, repo);

        // 1. Setup Global Cache Path
        let home_dir = dirs::home_dir().ok_or_else(|| AppError::System("Cannot determine home directory for cache".to_string()))?;
        let cache_dir = home_dir.join(".arsync").join("cache").join(owner).join(repo);

        // 2. Clone bare repository or fetch updates
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).map_err(AppError::Io)?;
            let mut clone_cmd = Command::new("git");
            clone_cmd.arg("clone").arg("--bare").arg("--filter=blob:none").arg(&url).arg(&cache_dir);
            let status = clone_cmd.status().map_err(|e| AppError::System(format!("Failed to execute git clone --bare: {}", e)))?;
            if !status.success() {
                return Err(AppError::System(format!("Git clone failed for {}", url)));
            }
        } else {
            let mut fetch_cmd = Command::new("git");
            fetch_cmd.current_dir(&cache_dir).arg("fetch").arg("origin");
            let status = fetch_cmd.status().map_err(|e| AppError::System(format!("Failed to execute git fetch: {}", e)))?;
            if !status.success() {
                // We ignore fetch failures if offline, but ideally log a warning
                eprintln!("Warning: Failed to fetch updates from {}, using local cache.", url);
            }
        }

        // 3. Create a temporary worktree to extract files
        let tmp_dir = tempfile::tempdir().map_err(|e| AppError::System(format!("Failed to create temp dir: {}", e)))?;
        let tmp_worktree = tmp_dir.path().to_path_buf();
        
        let mut worktree_cmd = Command::new("git");
        worktree_cmd.current_dir(&cache_dir).arg("worktree").arg("add").arg("-d").arg(&tmp_worktree);
        
        // Prioritize target_commit (from lockfile) over branch_or_tag
        if let Some(commit) = target_commit {
            worktree_cmd.arg(commit);
        } else if let Some(b) = branch_or_tag {
            worktree_cmd.arg(b);
        } else {
            // Default branches might differ, so we shouldn't pass anything to let git worktree figure it out,
            // or we explicitly checkout HEAD
            worktree_cmd.arg("HEAD");
        }

        let status = worktree_cmd.status().map_err(|e| AppError::System(format!("Failed to add working tree: {}", e)))?;
        if !status.success() {
             return Err(AppError::System(format!("Failed to checkout branch/tag/commit for {}", url)));
        }

        // --- Get the resolved commit hash from the worktree ---
        let rev_cmd = Command::new("git").current_dir(&tmp_worktree).arg("rev-parse").arg("HEAD").output();
        let resolved_commit = match rev_cmd {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            },
            _ => "unknown".to_string() // Fallback, though extremely unlikely to hit if worktree succeeded
        };

        // 4. Sparse-checkout if subfolder exists
        if !subfolder.is_empty() {
             let mut sparse_cmd = Command::new("git");
             sparse_cmd.current_dir(&tmp_worktree).arg("sparse-checkout").arg("set").arg(subfolder);
             let sparse_status = sparse_cmd.status().map_err(|e| AppError::System(format!("Failed to execute git sparse-checkout: {}", e)))?;
             if !sparse_status.success() {
                 let _ = Command::new("git").current_dir(&cache_dir).arg("worktree").arg("remove").arg("-f").arg(&tmp_worktree).status();
                 return Err(AppError::System(format!("Git sparse-checkout failed for {}", subfolder)));
             }
        }

        // 5. Move the fetched folder to the destination
        let src_path = if subfolder.is_empty() {
            tmp_worktree.clone()
        } else {
            tmp_worktree.join(subfolder)
        };

        if !src_path.exists() {
            let _ = Command::new("git").current_dir(&cache_dir).arg("worktree").arg("remove").arg("-f").arg(&tmp_worktree).status();
            return Err(AppError::System(format!("Source path '{}' not found in repository", subfolder)));
        }

        if dest.exists() {
            std::fs::remove_dir_all(dest).map_err(AppError::Io)?;
        }
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(AppError::Io)?;
        }

        let mut options = CopyOptions::new();
        options.copy_inside = true;
        
        let result = copy(&src_path, dest, &options);

        let _ = Command::new("git").current_dir(&cache_dir).arg("worktree").arg("remove").arg("-f").arg(&tmp_worktree).status();

        result.map_err(|e| AppError::System(format!("Failed to copy directory: {}", e)))?;

        if subfolder.is_empty() {
           let dest_git = dest.join(".git");
           if dest_git.exists() {
               let _ = if dest_git.is_dir() { std::fs::remove_dir_all(&dest_git) } else { std::fs::remove_file(&dest_git) };
           }
        }

        Ok(resolved_commit)
    }

    fn resolve_remote_hash(&self, source: &SkillSource) -> Result<String, AppError> {
        let mut source_str = source.as_str();
        let mut branch_or_tag: Option<&str> = None;

        if let Some(idx) = source_str.find('@') {
            branch_or_tag = Some(&source_str[idx + 1..]);
            source_str = &source_str[..idx];
        } else if let Some(idx) = source_str.find('#') {
            branch_or_tag = Some(&source_str[idx + 1..]);
            source_str = &source_str[..idx];
        }

        let mut parts = source_str.splitn(3, '/');
        let owner = parts.next().ok_or_else(|| AppError::System("Invalid source: missing owner".to_string()))?;
        let repo = parts.next().ok_or_else(|| AppError::System("Invalid source: missing repo".to_string()))?;

        let url = format!("https://github.com/{}/{}.git", owner, repo);
        let ref_name = branch_or_tag.unwrap_or("HEAD");

        let output = Command::new("git")
            .arg("ls-remote")
            .arg(&url)
            .arg(ref_name)
            .output()
            .map_err(|e| AppError::System(format!("Failed to execute git ls-remote: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::System(format!("git ls-remote failed for {}", url)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let hash = stdout.lines()
            .next()
            .and_then(|line| line.split_whitespace().next())
            .unwrap_or("unknown")
            .to_string();

        Ok(hash)
    }
}

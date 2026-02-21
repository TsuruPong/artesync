use std::path::Path;
use crate::core::domain::skill::SkillSource;
use crate::core::domain::error::AppError;

pub trait SkillFetcher {
    /// Returns the resolved Git commit hash that was fetched/copied
    fn fetch(&self, source: &SkillSource, dest: &Path, target_commit: Option<&str>) -> Result<String, AppError>;

    /// Resolves the latest remote commit hash for the given source without copying files.
    /// Used by `update` to skip expensive I/O when hashes already match.
    fn resolve_remote_hash(&self, source: &SkillSource) -> Result<String, AppError>;
}

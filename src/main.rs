pub mod cli;
pub mod core;
pub mod infra;

use std::env;
use clap::Parser;
use colored::Colorize;
use cli::parser::{Cli, Commands};
use crate::infra::fs::local::LocalFileSystem;
use crate::infra::manifest::file_repo::FileManifestRepository;
use crate::infra::manifest::lockfile_repo::FileLockfileRepository;
use crate::infra::git::fetcher::CliGitFetcher;
use crate::core::usecase::init::InitUseCase;
use crate::core::usecase::install::InstallUseCase;
use crate::core::usecase::uninstall::UninstallUseCase;
use crate::core::usecase::list::ListUseCase;
use crate::core::usecase::update::UpdateUseCase;

/// Resolves the install source string from a combination of shorthand positional argument and explicit flags.
/// Returns `Ok(None)` for bare `arsync install` (environment restore), `Ok(Some(source))` for a resolved source,
/// or `Err(message)` if flags are used incorrectly.
fn resolve_install_source(
    source: &Option<String>,
    owner: &Option<String>,
    repository: &Option<String>,
    branch: &Option<String>,
    tag: &Option<String>,
    path: &Option<String>,
) -> Result<Option<String>, String> {
    if let Some(s) = source {
        let mut path_str = s.as_str();
        let mut current_ref = String::new();

        // Extract ref from shorthand (end of string)
        if let Some(idx) = path_str.find('@') {
            current_ref = path_str[idx + 1..].to_string();
            path_str = &path_str[..idx];
        } else if let Some(idx) = path_str.find('#') {
            current_ref = path_str[idx + 1..].to_string();
            path_str = &path_str[..idx];
        }

        let mut parts = path_str.splitn(3, '/');
        let mut current_owner = parts.next().unwrap_or("").to_string();
        let mut current_repo = parts.next().unwrap_or("").to_string();
        let mut current_path = parts.next().unwrap_or("").to_string();

        // Apply explicit flag overrides
        if let Some(o) = owner { current_owner = o.clone(); }
        if let Some(r) = repository { current_repo = r.clone(); }
        if let Some(p) = path { current_path = p.clone(); }
        
        if let Some(b) = branch { 
            current_ref = b.clone(); 
        } else if let Some(t) = tag { 
            current_ref = t.clone(); 
        }

        let mut composed = format!("{}/{}", current_owner, current_repo);
        if !current_path.is_empty() {
            composed = format!("{}/{}", composed, current_path);
        }
        if !current_ref.is_empty() {
            let sep = if branch.is_some() || (!current_ref.is_empty() && s.contains('#')) { "#" } else { "@" };
            composed = format!("{}{}{}", composed, sep, current_ref);
        }
        
        Ok(Some(composed))
    } else {
        // No shorthand. Check if explicit flags are provided.
        if owner.is_some() || repository.is_some() || branch.is_some() || tag.is_some() || path.is_some() {
            if owner.is_none() || repository.is_none() {
                return Err("When using explicit flags without a positional source argument, both --owner and --repository must be provided.".to_string());
            }
            
            let mut composed = format!("{}/{}", owner.as_ref().unwrap(), repository.as_ref().unwrap());
            if let Some(p) = path {
                composed = format!("{}/{}", composed, p);
            }
            if let Some(b) = branch {
                composed = format!("{}#{}", composed, b);
            } else if let Some(t) = tag {
                composed = format!("{}@{}", composed, t);
            }
            Ok(Some(composed))
        } else {
            // Bare `arsync install` — restore environment from lockfile
            Ok(None)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    
    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    let fs = LocalFileSystem::new();
    let manifest_repo = FileManifestRepository::new();
    let lockfile_repo = FileLockfileRepository::new();
    let git_fetcher = CliGitFetcher::new();
    
    let result = match &cli.command {
        Commands::Init => {
            let usecase = InitUseCase::new(&manifest_repo, &lockfile_repo, &fs);
            usecase.execute(&current_dir)
        },
        Commands::Install { source, owner, repository, branch, tag, path } => {
            let final_source = match resolve_install_source(source, owner, repository, branch, tag, path) {
                Ok(s) => s,
                Err(msg) => {
                    eprintln!("{}", msg.red().bold());
                    std::process::exit(1);
                }
            };

            let usecase = InstallUseCase::new(&manifest_repo, &git_fetcher, &lockfile_repo);
            usecase.execute(&current_dir, final_source)
        },
        Commands::Uninstall { skill_name } => {
            let usecase = UninstallUseCase::new(&manifest_repo, &fs, &lockfile_repo);
            usecase.execute(&current_dir, skill_name)
        },
        Commands::List => {
            let usecase = ListUseCase::new(&manifest_repo);
            match usecase.execute(&current_dir) {
                Ok(skills) => {
                    if skills.is_empty() {
                        println!("No skills installed.");
                    } else {
                        println!("Installed skills:");
                        for (name, source) in skills {
                            println!("- {}: {}", name, source);
                        }
                    }
                    Ok(())
                },
                Err(e) => Err(e),
            }
        },
        Commands::Update { skill_name } => {
            let usecase = UpdateUseCase::new(&manifest_repo, &git_fetcher, &lockfile_repo);
            usecase.execute(&current_dir, skill_name.as_deref())
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_shorthand_only() {
        let result = resolve_install_source(
            &Some("owner/repo/path#main".to_string()),
            &None, &None, &None, &None, &None,
        );
        assert_eq!(result.unwrap(), Some("owner/repo/path#main".to_string()));
    }

    #[test]
    fn test_resolve_explicit_flags_only() {
        let result = resolve_install_source(
            &None,
            &Some("myowner".to_string()),
            &Some("myrepo".to_string()),
            &Some("dev".to_string()),
            &None,
            &Some("src/skills".to_string()),
        );
        assert_eq!(result.unwrap(), Some("myowner/myrepo/src/skills#dev".to_string()));
    }

    #[test]
    fn test_resolve_bare_install() {
        let result = resolve_install_source(&None, &None, &None, &None, &None, &None);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_resolve_flags_without_owner_errors() {
        let result = resolve_install_source(
            &None, &None, &Some("myrepo".to_string()), &None, &None, &None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_shorthand_with_flag_override() {
        let result = resolve_install_source(
            &Some("owner/repo#dev".to_string()),
            &None, &None,
            &Some("main".to_string()), // Override branch
            &None, &None,
        );
        assert_eq!(result.unwrap(), Some("owner/repo#main".to_string()));
    }
}


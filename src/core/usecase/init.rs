use std::path::Path;
use crate::core::domain::manifest::Manifest;
use crate::core::domain::skill::SkillName;
use crate::core::domain::error::AppError;
use crate::core::port::manifest_repository::ManifestRepository;
use crate::core::port::file_system::FileSystem;

pub struct InitUseCase<'a, M: ManifestRepository, F: FileSystem> {
    manifest_repo: &'a M,
    file_system: &'a F,
}

impl<'a, M: ManifestRepository, F: FileSystem> InitUseCase<'a, M, F> {
    pub fn new(manifest_repo: &'a M, file_system: &'a F) -> Self {
        Self { manifest_repo, file_system }
    }

    pub fn execute(&self, dir: &Path) -> Result<(), AppError> {
        let manifest_path = dir.join("skills.arsync");
        
        if self.file_system.exists(&manifest_path) {
            return Err(AppError::System("Manifest already exists in this directory".to_string()));
        }

        use std::io::{self, Write};
        use colored::Colorize;

        let default_name = dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-skill");

        // Simple sanitization for the default suggestion
        let sanitized_default = default_name.to_lowercase().replace('_', "-");

        // Prompt for Name
        print!("name: ({}) ", sanitized_default.cyan());
        io::stdout().flush().unwrap();
        
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name).unwrap();
        let input_name = input_name.trim();
        
        // Use user input if provided and valid, otherwise fallback to sanitized default
        let final_name_str = if input_name.is_empty() {
            sanitized_default.to_string()
        } else {
            input_name.to_string()
        };

        let skill_name = SkillName::new(&final_name_str)
            .map_err(|e| AppError::System(format!("Invalid skill name: {}", e)))?;

        // Prompt for Description
        print!("description: ");
        io::stdout().flush().unwrap();
        
        let mut input_desc = String::new();
        io::stdin().read_line(&mut input_desc).unwrap();
        let final_desc = input_desc.trim().to_string();
            
        let manifest = Manifest::new(skill_name, final_desc);
        
        self.manifest_repo.save(&manifest_path, &manifest)?;
        
        println!("\n{} Created skills.arsync", "✔".green().bold());
        Ok(())
    }
}

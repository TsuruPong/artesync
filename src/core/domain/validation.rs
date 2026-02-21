use std::path::Path;
use std::fs;
use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    name: Option<String>,
}

pub fn validate_skill_soft(skill_dir: &Path, expected_dir_name: &str) {
    let skill_md_path = skill_dir.join("SKILL.md");

    // 1. Check if SKILL.md exists
    if !skill_md_path.exists() {
        println!(
            "  {} SKILL.md not found. This might not be a valid Agent Skill.",
            "⚠️  Warning:".yellow().bold()
        );
        return;
    }

    // 2. Read and parse frontmatter
    if let Ok(content) = fs::read_to_string(&skill_md_path) {
        if content.starts_with("---") {
            // Find the end of the frontmatter block
            if let Some(end_idx) = content[3..].find("---") {
                let frontmatter_str = &content[3..end_idx + 3];
                
                match serde_yaml::from_str::<SkillFrontmatter>(frontmatter_str) {
                    Ok(frontmatter) => {
                        // 3. Name field missing
                        if frontmatter.name.is_none() {
                            println!(
                                "  {} 'name' field missing in SKILL.md frontmatter.",
                                "⚠️  Warning:".yellow().bold()
                            );
                            return;
                        }

                        // 4. Directory name mismatch
                        let actual_name = frontmatter.name.unwrap();
                        if actual_name != expected_dir_name {
                            println!(
                                "  {} Directory name mismatch. Expected '{}' to match frontmatter name '{}'. AI agents may fail to recognize this skill.",
                                "⚠️  Warning:".yellow().bold(),
                                expected_dir_name,
                                actual_name
                            );
                        }
                    }
                    Err(_) => {
                        println!(
                            "  {} Failed to parse YAML frontmatter in SKILL.md.",
                            "⚠️  Warning:".yellow().bold()
                        );
                    }
                }
            } else {
                println!(
                    "  {} Invalid frontmatter format in SKILL.md (missing closing '---').",
                    "⚠️  Warning:".yellow().bold()
                );
            }
        } else {
            println!(
                "  {} SKILL.md is missing YAML frontmatter at the top of the file.",
                "⚠️  Warning:".yellow().bold()
            );
        }
    } else {
        println!(
            "  {} Failed to read SKILL.md.",
            "⚠️  Warning:".yellow().bold()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_validate_soft_no_warnings() {
        let dir = tempdir().unwrap();
        let skill_md = dir.path().join("SKILL.md");
        let mut file = File::create(skill_md).unwrap();
        writeln!(file, "---\nname: my-skill\ndescription: Test\n---\nBody").unwrap();

        // This should not panic and ideally print nothing (or we just ensure it executes cleanly)
        validate_skill_soft(dir.path(), "my-skill");
    }

    // Since validation just prints warnings, we can't easily assert on output without intercepting stdout,
    // but we can ensure the code doesn't crash on various malformed inputs.
    
    #[test]
    fn test_validate_soft_missing_file() {
        let dir = tempdir().unwrap();
        validate_skill_soft(dir.path(), "my-skill");
    }

    #[test]
    fn test_validate_soft_malformed_yaml() {
        let dir = tempdir().unwrap();
        let skill_md = dir.path().join("SKILL.md");
        let mut file = File::create(skill_md).unwrap();
        writeln!(file, "---\nname: : : invalid yaml\n---\nBody").unwrap();
        
        validate_skill_soft(dir.path(), "my-skill");
    }

    #[test]
    fn test_validate_soft_mismatched_name() {
        let dir = tempdir().unwrap();
        let skill_md = dir.path().join("SKILL.md");
        let mut file = File::create(skill_md).unwrap();
        writeln!(file, "---\nname: wrong-name\n---\nBody").unwrap();
        
        validate_skill_soft(dir.path(), "my-skill");
    }
}

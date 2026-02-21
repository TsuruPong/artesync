use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "arsync")]
#[command(about = "ArteSync - Agent Skill Synchronizer", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new skills.arsync manifest in the current directory
    Init,

    /// Install skills from the manifest, or a specific skill if provided
    Install {
        /// Optional shorthand skill source (e.g. owner/repo, owner/repo/path#branch)
        source: Option<String>,

        /// Explicit GitHub owner/organization name
        #[arg(long)]
        owner: Option<String>,

        /// Explicit repository name
        #[arg(long, alias = "repo")]
        repository: Option<String>,

        /// Specific branch to check out
        #[arg(long, conflicts_with = "tag")]
        branch: Option<String>,

        /// Specific tag to check out
        #[arg(long, conflicts_with = "branch")]
        tag: Option<String>,

        /// Specific source directory path within the repository
        #[arg(long)]
        path: Option<String>,
    },

    /// Uninstall a specific skill by its name/key
    Uninstall {
        /// The name or key of the skill to uninstall
        skill_name: String,
    },

    /// List all installed skills currently in the manifest
    List,

    /// Update a specific skill, or all skills if none specified
    Update {
        /// Optional skill name to update
        skill_name: Option<String>,
    },
}

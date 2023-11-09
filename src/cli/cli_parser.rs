use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Jira Branches
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Jira username
    #[arg(short, long, global = true)]
    pub username: Option<String>,

    /// Jira password
    #[arg(short, long, global = true)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure Jira Branches
    Configure {
        /// Sets configuration from config file path
        #[arg(short, long, value_name = "FILE", global = true)]
        file: Option<PathBuf>,
    },
    /// Checkout branch for a Jira issue
    Checkout {
        /// Jira issue ID or URL
        #[arg()]
        issue: String,
    },
    /// List branches for a Jira issue
    Ls {
        /// Jira issue ID or URL
        #[arg()]
        issue: String,
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
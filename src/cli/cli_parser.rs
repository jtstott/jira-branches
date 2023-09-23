use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Jira Branches
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE", global = true)]
    // pub config: Option<PathBuf>,

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
    /// Checkout branch for a Jira issue
    Checkout {
        /// Jira issue ID or URL
        #[arg(short, long)]
        issue: String,
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
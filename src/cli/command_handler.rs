use std::error::Error;
use crate::cli::{checkout, ls};
use crate::cli::cli_parser::{self, Cli};
use crate::cli::configure;

pub async fn handle_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        cli_parser::Commands::Checkout { issue } => { checkout::handle(&cli, issue.to_owned()).await?; }
        cli_parser::Commands::Configure { file } => { configure::handle(file.to_owned())?; }
        cli_parser::Commands::Ls { issue } => { ls::handle(&cli, issue.to_owned()).await?; }
    }

    Ok(())
}

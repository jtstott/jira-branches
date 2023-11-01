use std::error::Error;
use crate::cli::checkout;
use crate::cli::cli_parser::{self, Cli};
use crate::cli::configure;

pub async fn handle_command(cli: Cli) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        cli_parser::Commands::Checkout { issue } => { checkout::handle(&cli, issue.to_owned()).await?; }
        cli_parser::Commands::Configure { file } => { configure::handle(file.to_owned())?; }
    }

    Ok(())
}

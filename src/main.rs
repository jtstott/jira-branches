mod jira;
mod cli;

use crate::cli::command_handler;

#[tokio::main]
async fn main() {
    command_handler::handle_command().await
}

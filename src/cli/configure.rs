use std::error::Error;
use std::path::PathBuf;
use crate::app_config::config_wizard::config_wizard;
use crate::app_config::file_parser::{read_config_file, write_user_config};

pub fn handle(file: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    if let Some(file) = file {
        let file_config = read_config_file(file.to_str())?;
        if let Some(uc) = file_config {
            write_user_config(&uc)?;
        }

        return Ok(())
    }

    config_wizard()?;

    Ok(())
}

use config::Config;
use crate::app_config::{AppConfig};

pub fn read_config_file() -> AppConfig {
    let dir = format!("{}/.config/jira-branches/config.json", dirs::home_dir().unwrap().as_path().to_str().unwrap());

    let config = Config::builder()
        .add_source(config::File::with_name(dir.as_str()))
        .build()
        .unwrap();

    config
        .try_deserialize::<AppConfig>()
        .unwrap()
}
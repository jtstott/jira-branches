use config::Config;
use crate::app_config::UserConfig;
use crate::jira::auth::JiraAuth;

pub fn read_config_file() -> UserConfig {
    let dir = format!("{}/.config/jira-branches/config.json", dirs::home_dir().unwrap().as_path().to_str().unwrap());

    let config = Config::builder()
        .add_source(config::File::with_name(dir.as_str()))
        .build()
        .unwrap();

    config
        .try_deserialize::<UserConfig>()
        .unwrap()
}

pub fn read_auth_file() -> JiraAuth {
    let dir = format!("{}/.config/jira-branches/auth.json", dirs::home_dir().unwrap().as_path().to_str().unwrap());

    let config = Config::builder()
        .add_source(config::File::with_name(dir.as_str()))
        .build()
        .unwrap();

    config
        .try_deserialize::<JiraAuth>()
        .unwrap()
}
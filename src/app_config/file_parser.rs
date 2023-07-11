use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::sync::Mutex;
use config::Config;
use crate::app_config::{Auth, AppConfig};

pub static AUTH: Mutex<Auth> = Mutex::new(Auth {
    user: String::new(),
    password: String::new()
});

pub fn read_config_file() -> AppConfig {
    let dir = format!("{}/.config/jira-branches/config.json", dirs::home_dir().unwrap().as_path().to_str().unwrap());

    let config = Config::builder()
        .add_source(config::File::with_name(dir.as_str()))
        .build()
        .unwrap();

    config
        .try_deserialize::<AppConfig>()
        .unwrap()

    // AUTH.lock().unwrap().user = config.auth.user.clone();
    // AUTH.lock().unwrap().password = config.auth.password.clone();
    //
    // println!("Config: {:#?}", config);
    // config
}
use std::collections::HashMap;
use colored::Colorize;
use crate::app_config::{AppConfig, Options, UserConfig};
use crate::jira::auth::JiraAuth;

pub fn config_wizard() -> AppConfig {
    println!("{}", "🪄 Jira Branches configuration wizard".bold().bright_blue());

    println!("{}", "\n🔐 First we need to authenticate with your Jira account...".bold().bright_blue());
    let auth = JiraAuth {
        user: get_required_config_value("Enter the email address for your Jira account", None),
        password: get_required_config_value("Enter your password for your Jira account", None),
    };

    println!("{}", "\n⚙️ Now let's setup your configuration...".bold().bright_blue());

    let config = UserConfig {
        base_url: get_required_config_value("Enter your Jira instance URL", None),
        branch_template: get_required_config_value(
            "Set your branch template",
            Some("The format branch names will be created to. Tokens can be used to represent Jira ticket variables."),
        ),
        options: Some(Options {
            id_prefix: get_optional_config_value(
                "Set an issue ID prefix?",
                Some("This option can be set if all Jira ticket IDs start with the same prefix. The prefix will be prepended to all issue ID arguments."),
            ),
            map_types: read_recursive_input_map(
                "Map Jira issue types?",
                Some("Allows Jira ticket types to be mapped to other values for branch name generation. \n\nEnter a mapping item in the following format: \nissueType:mappedValue\n"),
            ),
            // case: Some(HashMap::from([
            //     ("type".into(), "lower".into()),
            //     ("summary".into(), "lower".into()),
            // ])),
            case: read_recursive_input_map("Transform the case of values?", Some("Transform the raw Jira values to either upper or lower case. Valid values are either 'upper' or 'lower'.\n"))
        }),
    };

    AppConfig {
        auth,
        config,
    }
}

fn read_config_value(prompt: &str, desc: Option<&str>) -> Option<String> {
    println!("{}:", prompt.bold());
    if let Some(s) = desc { println!("{}", s.dimmed()) };
    read_string()
}

fn get_required_config_value(prompt: &str, desc: Option<&str>) -> String {
    let value = read_config_value(prompt, desc);
    value.unwrap_or_else(|| {
        println!("{}", "This option is required, please enter a value to continue".red());
        get_required_config_value(prompt, desc)
    })
}

fn get_optional_config_value(prompt: &str, desc: Option<&str>) -> Option<String> {
    let optional_prompt = format!("{} {}", prompt.bold(), "(optional)".dimmed());
    read_config_value(optional_prompt.as_str(), desc)
}

fn read_recursive_input_map(prompt: &str, desc: Option<&str>) -> Option<HashMap<String, String>> {
    let optional_prompt = format!("{} {}", prompt.bold(), "(optional)".dimmed());
    println!("{}:", optional_prompt);
    if let Some(s) = desc { println!("{}", s.dimmed()) };
    println!("{}", "Press the RETURN key (⏎) after entering a value to add another value, or press the RETURN key (⏎) without entering a value to continue".dimmed());

    let values = read_recursive(Vec::new(), 1);

    if values.is_empty() {
        return None
    }

    let mut map: HashMap<String, String> = HashMap::new();

    for v in values {
        let parts: Vec<&str> = v.split(':').collect();
        map.insert(parts[0].to_string(), parts[1].to_string());
    }

    Some(map)
}

fn read_recursive_input_map_provide_keys(prompt: &str, desc: Option<&str>, keys: Vec<String>) -> Option<HashMap<String, String>> {
    let optional_prompt = format!("{} {}", prompt.bold(), "(optional)".dimmed());
    println!("{}:", optional_prompt);
    if let Some(s) = desc { println!("{}", s.dimmed()) };

    let values = read_recursive(Vec::new(), 1);

    if values.is_empty() {
        return None
    }

    let mut map: HashMap<String, String> = HashMap::new();

    for v in values {
        let parts: Vec<&str> = v.split(':').collect();
        map.insert(parts[0].to_string(), parts[1].to_string());
    }

    Some(map)
}

fn read_recursive(mut values: Vec<String>, i: i32) -> Vec<String> {
    // println!("Enter value {}:", i);
    let value = read_string();
    match value {
        None => values,
        Some(v) => {
            values.push(v);
            read_recursive(values, i + 1)
        }
    }
}

fn read_string() -> Option<String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");

    match input.trim() {
        "" => None,
        &_ => Some(input.trim().into())
    }
}
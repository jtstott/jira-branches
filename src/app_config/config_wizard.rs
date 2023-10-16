use std::collections::{HashMap};
use std::error::Error;
use std::ops::Deref;
use colored::Colorize;
use inquire::{Confirm, InquireError, MultiSelect, Password, PasswordDisplayMode, required, Text};
use crate::app_config::{AppConfig, init, Options, UserConfig};
use crate::branch::template::get_template_tokens;
use crate::jira::auth::JiraAuth;

pub fn config_wizard() -> Result<AppConfig, Box<dyn Error>> {
    let existing_config = init::initialize_config(None)?;
    println!("{:?}", existing_config);
    println!("{}", "🪄 Jira Branches configuration wizard".bold().bright_blue());

    println!("{}", "\n🔐 First we need to authenticate with your Jira account...".bold().bright_blue());

    let user = Text::new("Enter the email address for your Jira account:")
        .with_help_message("Base URL of your Jira instance, E.g. https://my-org.atlassian.net")
        .with_validator(required!())
        .with_default(existing_config.auth.user.as_str())
        .prompt()?;
    let password = Password::new("Enter your password for your Jira account:")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(required!())
        .prompt()?;

    let auth = JiraAuth {
        user,
        password,
    };

    println!("{}", "\n⚙️ Now let's setup your configuration...".bold().bright_blue());

    let base_url = Text::new("Enter your Jira instance URL:")
        .with_default(existing_config.config.base_url.as_str())
        .prompt()?;

    let branch_template = Text::new("Set your branch template:")
        .with_default(existing_config.config.branch_template.as_str())
        .prompt()?;

    let options = existing_config.config.options;

    let id_prefix = Text::new("Set an issue ID prefix?:")
        .with_help_message("Optional - This option can be set if all Jira ticket IDs start with the same prefix. The prefix will be prepended to all issue ID arguments.")
        .with_default(options.unwrap_or_default().id_prefix.unwrap_or_default().as_str())
        .prompt_skippable()?;

    let do_map_types = Confirm::new("Map Jira issue types?")
        .with_placeholder("y")
        .with_default(true)
        .prompt()?;

    let mut map_types: HashMap<String, String> = HashMap::new();

    if do_map_types {
        println!("{}", "\nThe prompt will first ask for a Jira value to map, and then it will follow up by asking you for a value to map it to.\n".dimmed());
        let map_key = Text::new("Enter a Jira issue type to map:")
            .with_placeholder("e.g. Story")
            .with_help_message("Press the RETURN key (⏎) after entering a value to add another value, or press the RETURN key (⏎) without entering a value to continue")
            .prompt_skippable()?;

        if let Some(k) = map_key {
            let map_value = Text::new(format!("{}:", k).as_str())
                .with_placeholder("Enter value to map to")
                .with_help_message("Press the RETURN key (⏎) after entering a value to add another value, or press the RETURN key (⏎) without entering a value to continue")
                .prompt()?;

            map_types.insert(k, map_value);
        };
    };

    // let case_conf = &existing_config.config.options.as_ref().unwrap().case;

    let case_map = read_case_transform(
        &branch_template,
        // &existing_config
    )?;

    let config = UserConfig {
        base_url,
        branch_template,
        options: Some(Options {
            id_prefix,
            map_types: Some(map_types),
            case: Some(case_map),
        }),
    };

    let app_config = AppConfig {
        auth,
        config,
    };

    Ok(app_config)
}

fn read_case_transform(branch_template: &String) -> Result<HashMap<String, String>, InquireError> {
    let transform_lower_options = get_template_tokens(branch_template);
    let opts = Vec::from_iter(transform_lower_options.clone());

    // if let Some(c) = case_config.as_ref() {
    //     let vals = c.iter()
    //         .find_map(|(key, val)| if val == "lower" { Some(key) } else { None });
    //
    //     let keys = c.clone().into_keys().into_iter();
    //     for key in keys {
    //         println!("key: {}", key)
    //     }
    // }

    let to_lower = MultiSelect::new(
        case_transform_prompt("LOWER").as_str(),
        opts,
    )
        .with_default(&[1])
        .prompt()?;

    let to_upper = MultiSelect::new(
        case_transform_prompt("UPPER").as_str(),
        Vec::from_iter(&transform_lower_options - &to_lower.iter().cloned().collect()),
    )
        .prompt()?;

    let mut case_map: HashMap<String, String> = HashMap::new();
    for l in to_lower {
        case_map.insert(l, "lower".into());
    };
    for u in to_upper {
        case_map.insert(u, "upper".into());
    };

    Ok(case_map)
}

fn case_transform_prompt(case: &str) -> String {
    format!("Select token values to transform to {} case", case.bold())
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
        return None;
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
        return None;
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
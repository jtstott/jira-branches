use std::collections::{HashMap};
use std::error::Error;
use colored::Colorize;
use inquire::{Confirm, InquireError, MultiSelect, Password, PasswordDisplayMode, required, Text};
use inquire::error::InquireResult;
use crate::app_config::{AppConfig, config_loader, init, Options, UserConfig};
use crate::app_config::file_parser::write_config;
use crate::app_config::token_completer::JiraTokenCompleter;
use crate::branch::template::get_template_tokens;
use crate::jira::auth::JiraAuth;

pub fn config_wizard() -> Result<AppConfig, Box<dyn Error>> {
    println!("{}", "🪄 Jira Branches configuration wizard".bold().bright_blue());
    let existing_config = config_loader::load_user_config().unwrap_or_default();
    let existing_auth = config_loader::load_auth(None).unwrap_or_default();

    println!("{}", "\n🔐 First we need to authenticate with your Jira account...".bold().bright_blue());

    let user = wrap_text_prompt(
        Text::new("Enter the email address for your Jira account:")
            .with_validator(required!()),
        &existing_auth.user)?;

    let password = Password::new("Enter your password for your Jira account:")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(required!())
        .prompt()?;

    println!("{}", "\n⚙️ Now let's setup your configuration...".bold().bright_blue());

    let base_url = wrap_text_prompt(
        Text::new("Enter your Jira instance URL:")
            .with_help_message("Base URL of your Jira instance, E.g. https://my-org.atlassian.net")
            .with_validator(required!()),
        &existing_config.base_url,
    )?;

    let branch_template = wrap_text_prompt(
        Text::new("Set your branch template:")
            .with_autocomplete(JiraTokenCompleter::default())
            .with_validator(required!()),
        &existing_config.branch_template,
    )?;

    let options = existing_config.options;

    let id_prefix = Text::new("Set an issue ID prefix?:")
        .with_help_message("Optional - This option can be set if all Jira ticket IDs start with the same prefix. The prefix will be prepended to all issue ID arguments.")
        .with_default(options.clone().unwrap_or_default().id_prefix.unwrap_or_default().as_str())
        .prompt_skippable()?;

    let existing_map_types = options.clone().unwrap_or_default().map_types;

    let map_types_help = format!("Enter n to keep current mapping: {:?}", existing_map_types.as_ref());

    let do_map_types = Confirm::new("Map Jira issue types?")
        .with_placeholder("n")
        .with_default(false)
        .with_help_message(map_types_help.as_str())
        .prompt()?;

    let map_types = if do_map_types { read_map_types()? } else { existing_map_types };

    let case_map = read_case_transform(
        &branch_template,
        &options.unwrap_or_default().case,
    )?;

    let app_config = AppConfig {
        auth: JiraAuth {
            user,
            password,
        },
        config: UserConfig {
            base_url,
            branch_template,
            options: Some(Options {
                id_prefix,
                map_types,
                case: case_map,
            }),
        },
    };

    print_config(&app_config);
    write_config(&app_config)?;

    Ok(app_config)
}

fn wrap_text_prompt(input: Text, default: &String) -> InquireResult<String> {
    if !default.is_empty() {
        return input.with_default(default.as_str()).prompt();
    }

    input.prompt()
}

fn read_map_types() -> Result<Option<HashMap<String, String>>, InquireError> {
    let mut map_types: HashMap<String, String> = HashMap::new();
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

    Ok(Some(map_types))
}

fn read_case_transform(branch_template: &String, case_config: &Option<HashMap<String, String>>) -> Result<Option<HashMap<String, String>>, InquireError> {
    let transform_lower_options = get_template_tokens(branch_template);
    let opts = Vec::from_iter(transform_lower_options.clone());

    let mut case_map: HashMap<String, String> = HashMap::new();

    let mut lowers: Vec<&String> = Vec::new();
    if let Some(c) = case_config {
        c.iter()
            .filter(|(k, v)| v == &"lower")
            .for_each(|(k, v)| lowers.push(k));
    }

    if transform_lower_options.is_empty() { return Ok(None); };

    fn case_transform_prompt(case: &str) -> String {
        format!("Select token values to transform to {} case", case.bold())
    }

    if !transform_lower_options.is_empty() {
        let to_lower = MultiSelect::new(
            case_transform_prompt("LOWER").as_str(),
            opts,
        )
            .prompt()?;

        for l in &to_lower {
            case_map.insert(l.to_owned(), "lower".into());
        };

        let transform_upper_options = &transform_lower_options - &to_lower.iter().cloned().collect();
        if !transform_upper_options.is_empty() {
            let to_upper = MultiSelect::new(
                case_transform_prompt("UPPER").as_str(),
                Vec::from_iter(transform_upper_options),
            )
                .prompt()?;

            for u in to_upper {
                case_map.insert(u, "upper".into());
            };
        }
    }

    Ok(if case_map.is_empty() { None } else { Some(case_map) })
}

fn print_config(app_config: &AppConfig) {
    println!("{}", "\nYour updated Jira branches configuration:".bold());
    println!("{}", "🔐 Authentication".bright_blue().bold());
    display_value("Username", &app_config.auth.user);

    let password_display = if app_config.auth.password.is_empty() { "".into() } else { "*".repeat(app_config.auth.password.len()) };
    display_value("Password", &password_display);

    println!("{}", "\n⚙️ Configuration".bright_blue().bold());
    display_value("Base URL", &app_config.config.base_url);
    display_value("Branch template", &app_config.config.branch_template);

    if let Some(options) = &app_config.config.options {
        if let Some(id_prefix) = &options.id_prefix {
            display_value("ID Prefix", id_prefix);
        }

        if let Some(map_types) = &options.map_types {
            display_value("Map types", &format!("{:?}", map_types));
        }

        if let Some(case) = &options.case {
            display_value("Case transformations", &format!("{:?}", case));
        }
    }
    println!("{}", "\nJira branches successfully configured!".green().bold());
}

fn display_value(key: &str, value: &String) {
    if !value.is_empty() {
        println!("{}: {}", key.bold(), value);
    }
}
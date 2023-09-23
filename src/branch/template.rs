use std::collections::HashMap;
use crate::app_config::UserConfig;
use crate::branch::sanitizer;
use crate::jira::issue::JiraIssue;

pub fn interpret_branch_template(config: &UserConfig, issue: JiraIssue) -> String {
    let template_values = get_template_values(issue, config);

    let mut branch_template = config.branch_template.clone();

    for (key, value) in template_values {
        let to_replace = format!("[{}]", key);
        branch_template = branch_template.replace(to_replace.as_str(), format_jira_value(key, value, config).as_str());
    }

    sanitizer::remove_forbidden_chars(branch_template)
}

fn get_template_values(issue: JiraIssue, config: &UserConfig) -> HashMap<&'static str, String> {
    let mut template_values = HashMap::new();
    template_values.insert("id", issue.key);
    template_values.insert("type", map_type(config, &issue.fields.issuetype.name));
    template_values.insert("summary", issue.fields.summary);

    template_values
}

fn format_jira_value(key: &str, value: String, config: &UserConfig) -> String {
    let formatted_value = String::from(format_case(key, value, config).trim());
    match key {
        "summary" => sanitizer::replace_chars(formatted_value),
        "type" => formatted_value,
        _ => formatted_value
    }
}

fn format_case(key: &str, value: String, config: &UserConfig) -> String {
    if let Some(options) = &config.options {
        if let Some(case) = &options.case {
            return match case.get(key) {
                None => value,
                Some(k) => {
                    match k.as_str() {
                        "lower" => value.to_lowercase(),
                        "upper" => value.to_uppercase(),
                        &_ => value
                    }
                }
            };
        }
    }

    value
}

fn map_type(config: &UserConfig, issue_type: &String) -> String {
    if let Some(options) = &config.options {
        if let Some(mapped_types) = &options.map_types {
            return mapped_types.get(issue_type.as_str())
                .unwrap_or_else(|| mapped_types.get("*").unwrap_or(&issue_type)).clone()
        }
    };

    issue_type.to_string()
}

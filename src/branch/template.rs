use std::collections::HashMap;
use crate::app_config::AppConfig;
use crate::branch::sanitizer;
use crate::jira::issue::JiraIssue;

pub fn interpret_branch_template(config: &AppConfig, issue: JiraIssue) -> String {
    let template_values = get_template_values(issue);
    let mut branch_template = config.branch_template.clone();

    for (key, value) in template_values {
        let to_replace = format!("[{}]", key);
        branch_template = branch_template.replace(to_replace.as_str(), value.as_str());
    }

    branch_template = sanitizer::sanitize_branch(branch_template);
    println!("Template interpreted: {}", branch_template);

    branch_template
}

pub fn get_template_values(issue: JiraIssue) -> HashMap<&'static str, String> {
    let mut template_values = HashMap::new();
    template_values.insert("id", issue.key);
    template_values.insert("type", issue.fields.issuetype.name);
    template_values.insert("summary", issue.fields.summary);

    template_values
}



use std::collections::HashMap;
use crate::jira::client;
use serde::{Deserialize, Serialize};
use url::{Url};
use crate::app_config::{AppConfig, UserConfig};
use crate::jira::issue_url::parse_url;

pub async fn get_issue(issue_ref: &str, config: &AppConfig) -> Result<JiraIssue, String> {
    let id = extract_id(issue_ref)?;
    let ticket_id = prefix_id(id.as_str(), &config.config);
    let url = format!("/issue/{}?fields={}", ticket_id, build_fields_query_string());
    let response = client::make_request(
        url,
        config,
    ).await;

    match response?.json::<JiraIssue>().await {
        Ok(parsed) => { Ok(parsed) }
        Err(_) => {
            Err(String::from("Hm, the response didn't match the shape we expected."))
        }
    }
}

fn prefix_id(id: &str, config: &UserConfig) -> String {
    if let Some(options) = &config.options {
        if let Some(prefix) = &options.id_prefix {
            if !id.starts_with(prefix) {
                return prefix.to_string() + id;
            }
        }
    }

    id.to_string()
}

pub fn jira_fields() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("id", "id"),
        ("summary", "summary"),
        ("issuetype", "type"),
    ])
}

pub fn build_fields_query_string() -> String {
    let mut fields = jira_fields();
    fields.remove("id");
    let keys: Vec<&str> = fields.into_keys().collect();
    keys.to_owned().join(",")
}

pub fn extract_id(issue_ref: &str) -> Result<String, &str> {
    match Url::parse(issue_ref) {
        Ok(url) => parse_url(url),
        Err(_) => {
            Ok(issue_ref.to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JiraIssue {
    expand: String,
    id: String,
    #[serde(rename = "self")]
    jira_response_self: String,
    pub key: String,
    pub fields: Fields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub summary: String,
    pub issuetype: IssueType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    #[serde(rename = "self")]
    issuetype_self: String,
    id: String,
    pub description: String,
    icon_url: String,
    pub name: String,
    pub subtask: bool,
    avatar_id: i64,
    hierarchy_level: i64,
}
use crate::jira::client;
use serde::{Deserialize, Serialize};
use crate::app_config::{AppConfig, UserConfig};

pub async fn get_issue(id: &str, config: &AppConfig) -> Result<JiraIssue, String> {
    let ticket_id = prefix_id(id, &config.config);
    let url = format!("/issue/{}?fields=summary,issuetype", ticket_id);
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
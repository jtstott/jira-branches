use crate::jira::client;
use serde::{Deserialize, Serialize};
use crate::app_config::AppConfig;

pub async fn get_issue(id: &str, config: &AppConfig) -> JiraIssue {
    let url = format!("/issue/{}?fields=summary,issuetype", id);
    let response = client::make_request(
        url,
        &config.auth,
    ).await;

    // response.json::<JiraResponse>().await

    match response.json::<JiraIssue>().await {
        Ok(parsed) => { parsed }
        Err(_) => {
            println!("Hm, the response didn't match the shape we expected.");
            panic!("Can't keep going");
            // return None
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
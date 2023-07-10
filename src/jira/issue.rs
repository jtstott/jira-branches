use crate::jira::client;
use serde::{Deserialize, Serialize};
use crate::jira::auth::Auth;

pub async fn get_issue(id: &str, auth: &Auth) {
    let url = format!("/issue/{}?fields=summary,issuetype", id);
    let response = client::make_request(
        url,
        auth,
    ).await;

    match response.json::<JiraResponse>().await {
        Ok(parsed) => println!("Success! {:?}", parsed),
        Err(_) => {
            println!("Hm, the response didn't match the shape we expected.")
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JiraResponse {
    expand: String,
    id: String,
    #[serde(rename = "self")]
    jira_response_self: String,
    key: String,
    fields: Fields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    summary: String,
    issuetype: IssueType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    #[serde(rename = "self")]
    issuetype_self: String,
    id: String,
    description: String,
    icon_url: String,
    name: String,
    subtask: bool,
    avatar_id: i64,
    hierarchy_level: i64,
}
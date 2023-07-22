use reqwest::{IntoUrl, Response};
use http::header::{ACCEPT, CONTENT_TYPE};
use http::StatusCode;
use serde_json::Value;
use crate::app_config::AppConfig;
use crate::jira::error;

pub async fn make_request<U: IntoUrl>(path: U, config: &AppConfig) -> Result<Response, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/rest/api/3/{}", config.config.base_url, path.as_str());

    let response = client.get(url)
        .basic_auth(&config.auth.user, Some(&config.auth.password))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    let status = response.status();
    match status {
        StatusCode::OK => Ok(response),
        _ => {
            let json = response.json::<Value>().await.unwrap();
            let error = error::hande_error_response(json);
            Err(error)
        }
    }
}
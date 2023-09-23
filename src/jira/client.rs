use reqwest::{IntoUrl, Response};
use http::header::{ACCEPT, CONTENT_TYPE};
use http::StatusCode;
use serde_json::Value;
use crate::app_config::AppConfig;
use crate::jira::error;

pub async fn make_request<U: IntoUrl>(path: U, AppConfig { config, auth }: &AppConfig) -> Result<Response, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/rest/api/3/{}", config.base_url, path.as_str());

    let response = client.get(url)
        .basic_auth(&auth.user, Some(&auth.password))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await;

    match response {
        Ok(res) => handle_response(res).await,
        Err(error) => Err(error.to_string())
    }
}

async fn handle_response(response: Response) -> Result<Response, String> {
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
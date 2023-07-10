use reqwest::{IntoUrl, Response};
use http::header::{ACCEPT, CONTENT_TYPE};
use crate::jira::auth::Auth;
// use serde::{Deserialize, Serialize};

pub async fn make_request<U: IntoUrl>(path: U, auth: &Auth) -> Response {
    let client = reqwest::Client::new();
    let url = format!("https://netmanagement.atlassian.net/rest/api/3/{}", path.as_str());

    let response = client.get(url)
        .basic_auth(&auth.username, auth.password.clone())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    response

    //@TODO: Handle body serialization and/or errors.

    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //         return response.json()
    //         // match response.json::<JiraResponse>().await {
    //         //     Ok(parsed) => println!("Success! {:?}", parsed),
    //         //     Err(_) => {
    //         //         println!("Hm, the response didn't match the shape we expected.")
    //         //     }
    //         // };
    //     }
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Need to grab a new token");
    //     }
    //     other => {
    //         panic!("Uh oh! Something unexpected happened: {:?}", other);
    //     }
    // }
}
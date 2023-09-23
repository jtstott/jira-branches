use std::collections::HashMap;
use url::Url;

pub fn parse_url(url: Url) -> Result<String, &'static str> {
    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    if let Some(selected_issue) = params.get("selectedIssue"){
        return Ok(selected_issue.to_string())
    }

    if url.path().starts_with("/browse/") {
        if let Some(segments) =url.path_segments() {
            if let Some(item) = segments.last() {
                return Ok(item.to_string())
            }
        }
    }

    Err("Not a valid Jira issue URL")
}
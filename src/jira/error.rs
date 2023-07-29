use serde_json::Value;

pub fn hande_error_response(json: Value) -> String {
    if let Some(error_messages) = json.get("errorMessages") {
        let error_message = error_messages.get(0).unwrap();
        return map_error_message(Some(error_message));
    };

    if let Some(error_message) = json.get("errorMessage") {
        return map_error_message(Some(error_message));
    };

    map_error_message(None)
}

fn map_error_message(error_message: Option<&Value>) -> String {
    let default = "There was an error connecting to Jira";

    match error_message {
        None => String::from(default),
        Some(message_value) => {
            let message = message_value.as_str().unwrap_or(default);
            match message {
                "Site temporarily unavailable" => String::from("Jira is temporarily unavailable. Make sure the configured base_url is correct."),
                "Issue does not exist or you do not have permission to see it." => format!("{} Make sure your auth is correct and the issue ID provided exists.", message),
                _ => String::from(message)
            }
        }
    }
}
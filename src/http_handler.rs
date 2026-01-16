use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{Error, IntoResponse, Request};
use log::{error, info};
use serde_json::json;
use crate::auth_response::{is_not_authorized, AuthResponse};

pub async fn function_handler(client: Client, table_name: String, event: Request) -> Result<impl IntoResponse, Error> {
    info!("Received event: {:?}", event);
    let api_key_option = event.headers()
        .get("x-api-key")
        .and_then(|header| header.to_str().ok());

    let api_key = match api_key_option {
        Some(key) => key,
        None => return Ok(json!(is_not_authorized()))
    };

    let result = client
        .get_item()
        .table_name(table_name)
        .key("ApiKey", AttributeValue::S(api_key.to_string()))
        .send()
        .await;

    let is_authorized = match result {
        Ok(item) =>item.item.is_some(),
        Err(err) => {
            error!("{}", err);
            false
        },
    };

    Ok(json!(AuthResponse { is_authorized }))
}

use std::env;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
mod auth_response;

use http_handler::function_handler;
use log::info;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    info!("starting up");

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let table_name = env::var("DYNAMODB_AUTH_TABLE_NAME").expect("DYNAMODB_AUTH_TABLE_NAME must be set.");
    
    run(service_fn(move |event| { function_handler(client.clone(), table_name.clone(), event) })).await
}

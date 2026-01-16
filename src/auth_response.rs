use log::{error, info};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    #[serde(rename = "isAuthorized")]
    pub(crate) is_authorized: bool,
}

pub fn is_authorized() -> AuthResponse {
    info!("Request is authorized.");
    AuthResponse { is_authorized: true }
}

pub fn is_not_authorized() -> AuthResponse {
    error!("Request is not authorized.");
    AuthResponse { is_authorized: false}
}
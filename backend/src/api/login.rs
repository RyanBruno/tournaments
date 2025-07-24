use http::Request;
use http::Response;
use http::StatusCode;
use log::{info, warn};
use std::error::Error;

use crate::{PlatformModel, PlatformStore};
use models::LoginAttempt;

/// Authenticate a platform user.
///
/// Returns `UNAUTHORIZED` if credentials are incorrect or
/// `BAD_REQUEST` when a required field is empty.
pub fn login_route(
    _req: &Request<()>,
    platform_store: PlatformStore,
    email: String,
    password: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    info!("login attempt for {email}");

    if email.trim().is_empty() || password.trim().is_empty() {
        warn!("login request missing credentials");
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(b"{}".to_vec())?);
    }

    let user = platform_store
        .borrow_inner()
        .query_owned(format!("user-{email}"))?;

    let login_attempt = LoginAttempt { email, password };

    match user {
        Some(PlatformModel::User(user)) if user == login_attempt => {
            let json = serde_json::to_vec(&user)?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(b"{}".to_vec())?),
    }
}

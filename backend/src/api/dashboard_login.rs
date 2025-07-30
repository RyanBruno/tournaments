use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

use crate::not_found_route;
use crate::{generate as jwt_generate, DashboardModel, DashboardStore};
use log::{info, warn};
use models::LoginAttempt;

/// Authenticate a dashboard user by email and password.
/// Returns `NOT_FOUND` if the user does not exist or credentials do not match.
pub fn dashboard_login_route(
    _req: &Request<()>,
    dashboard_store: DashboardStore,
    email: String,
    password: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    info!("dashboard login attempt for {email}");

    // Query the user from the store using the provided email
    let user = dashboard_store
        .borrow_inner()
        .query_owned(format!("user-{email}"))?;

    let login_attempt = LoginAttempt { email, password };

    match user {
        Some(DashboardModel::User(u)) if u == login_attempt => {
            info!("dashboard login succeeded for {}", login_attempt.email);

            let token = jwt_generate(&login_attempt.email)?;
            let json = serde_json::to_vec(&serde_json::json!({ "token": token }))?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => {
            warn!("dashboard login failed for {}", login_attempt.email);
            not_found_route()
        }
    }
}

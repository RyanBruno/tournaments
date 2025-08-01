use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{verify, not_found_route, DashboardStore, DashboardModel, DashboardCommand};
use log::{info, warn};
use models::DashboardUserPatch;

/// Fetch the authenticated user's dashboard profile.
pub fn dashboard_profile_get_route(
    req: &Request<()>,
    dashboard_store: DashboardStore,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let claims = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.strip_prefix("Bearer ").unwrap_or(h))
        .and_then(|t| verify(t).ok());

    let Some(claims) = claims else {
        return Ok(
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(b"{}".to_vec())?,
        );
    };

    let key = format!("user-{}", claims.sub);
    info!("loading profile for {}", claims.sub);
    let model = dashboard_store.borrow_inner().query_owned(key.clone())?;
    match model {
        Some(DashboardModel::User(user)) => {
            let json = serde_json::to_vec(&user)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => {
            warn!("profile not found for {}", claims.sub);
            not_found_route()
        }
    }
}

/// Update the authenticated user's profile using the provided password.
pub fn dashboard_profile_patch_route(
    req: &Request<()>,
    mut dashboard_store: DashboardStore,
    password: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let claims = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.strip_prefix("Bearer ").unwrap_or(h))
        .and_then(|t| verify(t).ok());

    let Some(claims) = claims else {
        return Ok(
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(b"{}".to_vec())?,
        );
    };

    info!("updating profile for {}", claims.sub);
    let patch = DashboardUserPatch {
        password: if password.is_empty() { None } else { Some(password) },
    };
    dashboard_store.command(&DashboardCommand::UpdateUser((claims.sub.clone(), patch)))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"updated\"}".to_vec())?)
}

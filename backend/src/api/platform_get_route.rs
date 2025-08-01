use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{not_found_route, verify, PlatformModel, PlatformStore};
use log::{info, warn};

/// Fetch platform information for the given tenant id.
pub fn platform_get_route(
    req: &Request<()>,
    platform_store: PlatformStore,
    tenant_id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let auth_ok = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.strip_prefix("Bearer ").unwrap_or(h))
        .and_then(|t| verify(t).ok())
        .is_some();

    if !auth_ok {
        return Ok(
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(b"{}".to_vec())?,
        );
    }

    info!("loading platform {tenant_id}");
    let platform = platform_store
        .borrow_inner()
        .query_owned(format!("platform-{tenant_id}"))?;

    match platform {
        Some(PlatformModel::Platform(p)) => {
            let json = serde_json::to_vec(&p)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => {
            warn!("platform {tenant_id} not found");
            not_found_route()
        }
    }
}

use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{verify, PlatformCommand, PlatformStore};
use log::info;
use models::Platform;

/// Create a new platform entry from a JSON request body.
/// Returns a `CREATED` response on success.

pub fn platform_create_route(
    req: Request<Vec<u8>>,
    mut platform_store: PlatformStore,
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

    // Deserialize the request body into a `Platform` model
    let platform: Platform = serde_json::from_slice(req.body())?;
    info!("creating platform {}", platform.tenant_id);

    // Persist the platform in the store
    platform_store.command(&PlatformCommand::CreatePlatform(platform))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"created\"}".to_vec())?)
}

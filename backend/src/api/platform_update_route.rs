use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{verify, PlatformCommand, PlatformStore};
use log::{info, warn};
use models::PlatformPatch;

/// Update an existing platform using JSON patch data.
/// Returns `OK` when the update has been persisted.

pub fn platform_update_route(
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
                .body(b"{}".to_vec())?,
        );
    }

    // Deserialize the patch from the request body
    let patch: PlatformPatch = serde_json::from_slice(req.body())?;
    info!("updating platform {}", patch.tenant_id);

    if patch.community_name.is_none()
        && patch.community_description.is_none()
        && patch.platform_url.is_none()
    {
        warn!("received empty platform patch for {}", patch.tenant_id);
    }

    platform_store.command(&PlatformCommand::UpdatePlatform(patch))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"updated\"}".to_vec())?)
}

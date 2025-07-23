use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{PlatformCommand, PlatformStore};
use models::PlatformPatch;

pub fn platform_update_route(
    req: Request<Vec<u8>>,
    mut platform_store: PlatformStore,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let patch: PlatformPatch = serde_json::from_slice(req.body())?;
    platform_store.command(&PlatformCommand::UpdatePlatform(patch))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"updated\"}".to_vec())?)
}

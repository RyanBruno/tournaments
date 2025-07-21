use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{PlatformCommand, PlatformStore};
use models::Platform;

pub fn platform_create_route(
    req: Request<Vec<u8>>,
    mut platform_store: PlatformStore,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let platform: Platform = serde_json::from_slice(req.body())?;
    platform_store.command(&PlatformCommand::CreatePlatform(platform))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"created\"}".to_vec())?)
}

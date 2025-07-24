use http::{Request, Response, StatusCode};
use log::{error, info, warn};
use std::error::Error;

use crate::{RegistrationCommand, RegistrationStore};
use models::Registration;

/// Register a user for a given event. Returns the registration record as JSON.
/// A `BAD_REQUEST` response is returned when parameters are empty.
pub fn register_event_route(
    _req: &Request<()>,
    mut store: RegistrationStore,
    event_id: String,
    email: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    info!("register `{email}` for event `{event_id}`");

    if event_id.trim().is_empty() || email.trim().is_empty() {
        warn!("invalid registration parameters");
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(b"{}".to_vec())?);
    }

    let registration = Registration {
        id: format!("{}-{}", event_id, email),
        event_id,
        email,
    };

    if let Err(e) = store.command(&RegistrationCommand::CreateRegistration(
        registration.clone(),
    )) {
        error!("failed to create registration: {e}");
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(b"{}".to_vec())?);
    }

    let json = serde_json::to_vec(&registration)?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(json)?)
}

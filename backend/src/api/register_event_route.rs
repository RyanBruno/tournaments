use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::{RegistrationCommand, RegistrationStore};
use models::Registration;

pub fn register_event_route(
    _req: &Request<()>,
    mut store: RegistrationStore,
    event_id: String,
    email: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let registration = Registration {
        id: format!("{}-{}", event_id, email),
        event_id,
        email,
    };

    store.command(&RegistrationCommand::CreateRegistration(
        registration.clone(),
    ))?;

    let json = serde_json::to_vec(&registration)?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(json)?)
}

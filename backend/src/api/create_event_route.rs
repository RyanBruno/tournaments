use crate::{Event, EventPatch, IndexedStoreHandle};
use http::{Request, Response, StatusCode};
use log::info;
use std::error::Error;

/// Create a new event for a tenant from JSON.
/// The event is persisted in the provided indexed store.

pub fn create_event_route(
    req: Request<Vec<u8>>,
    event_store: IndexedStoreHandle<Event, EventPatch, String>,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    // Read body bytes and parse JSON into an `Event`
    let event: Event = serde_json::from_slice(req.body())?;
    info!("creating event {} for {}", event.id, event.tenant_id);

    // Create the event in the store
    event_store.create(event.id.clone(), event)?;

    // Return success response
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"created\"}".to_vec())?)
}

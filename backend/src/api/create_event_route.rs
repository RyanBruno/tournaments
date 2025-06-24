use http::{Request, Response, StatusCode};
use crate::{Event, EventPatch, IndexedStoreHandle};
use std::error::Error;

pub fn create_event_route(
  req: Request<Vec<u8>>,
  event_store: IndexedStoreHandle<Event, EventPatch, String>
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  // Read body bytes
  let body = req.body();

  // Parse JSON into Event
  let event: Event = serde_json::from_slice(body)?;

  // Create the event in store
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

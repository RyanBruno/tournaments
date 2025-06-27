use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;
use crate::not_found_route;

use crate::{Event, EventPatch, IndexedStoreHandle};

pub fn event_details_route(
  req: Request<()>,
  event_store: IndexedStoreHandle<Event, EventPatch, String>
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  let event = event_store.borrow_inner()
    .read_owned("1".to_string());

  match event {
    Ok(Some(event)) => {
      let json = serde_json::to_vec(&event)?;

      Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(json)?)

    },
    _ => {
      not_found_route(req)
    }
  }
}
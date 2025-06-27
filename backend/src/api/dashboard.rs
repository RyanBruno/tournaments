use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

use serde::{Deserialize, Serialize};
use crate::{Event, EventPatch, IndexedStoreHandle};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardApi {
  pub announcment: String,
  pub name: String,
  pub events: Vec<Event>,
}

pub fn dashboard_route(
  _req: Request<()>,
  event_store: IndexedStoreHandle<Event, EventPatch, String>,
  tenant_id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  let events = event_store.borrow_inner()
    .query_owned_entities(&tenant_id);

  let dummy_data = DashboardApi {
    name: "Bucket Golf Leagues".to_string(),
    announcment: "⛳️ New summer leagues of bucket golf just dropped. Rally your crew and start swinging!".to_string(),
    events,
  };

  let json = serde_json::to_vec(&dummy_data)?;

  Ok(Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .header("Access-Control-Allow-Origin", "*")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .body(json)?)
}
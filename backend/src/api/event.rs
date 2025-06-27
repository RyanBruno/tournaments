use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

use crate::{Event, EventPatch, IndexedStoreHandle};

pub fn event_details_route(
  _req: Request<()>,
  event_store: IndexedStoreHandle<Event, EventPatch, String>
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  //let events = event_store.query(&"bucket-golf".to_string());

  let dummy_data = Event {
    tenent_id: "bucket-golf".into(),
    id: "1".into(),
    name: "Arlington Bucket Golf League".into(),
    location: "Quincy Park, VA".into(),
    date: "Saturday, July 13 – 3:00 PM".into(),
    image: "/static/bucket-golf.jpg".into(),
    banner: Some("⚡ Almost Sold Out".into()),
    upsell: Some("Only 2 slots left".into()),
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
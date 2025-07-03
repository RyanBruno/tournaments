use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;
use crate::not_found_route;

use crate::DashboardStore;

pub fn event_details_route(
  _req: &Request<()>,
  dashboard_store: DashboardStore,
  id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  let event = dashboard_store.borrow_inner()
    .query_owned(id.clone())?;

  match event {
    Some(crate::DashboardModel::Event(event)) => {
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
      not_found_route()
    }
  }
}
use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

use serde::{Deserialize, Serialize};
use crate::not_found_route;
use crate::{DashboardStore, DashboardModel};
use models::{Event};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardApi {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
}

pub fn dashboard_route(
  _req: &Request<()>,
  dashboard_store: DashboardStore,
  tenant_id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  let dashboard = dashboard_store.borrow_inner()
    .query_owned(tenant_id.clone())?;

  match dashboard {
    Some(DashboardModel::DashboardData(dashboard)) => {
      let active = dashboard.events.iter().filter(|e| e.active).cloned().collect();
      let json: Vec<u8> = serde_json::to_vec(&(dashboard + active))?;

      Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(json)?)
    },
    _ => not_found_route()
  }

  /*let dummy_data = DashboardApi {
    name: "Bucket Golf Leagues".to_string(),
    announcement: "⛳️ New summer leagues of bucket golf just dropped. Rally your crew and start swinging!".to_string(),
    events,
  };*/

}
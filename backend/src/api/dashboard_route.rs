use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

use crate::not_found_route;
use crate::{DashboardModel, DashboardStore};
use log::{info, warn};
use models::Event;
use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
/// API response model for the dashboard endpoint
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardApi {
    pub announcement: String,
    pub name: String,
    pub events: Vec<Event>,
}

/// Fetch the dashboard and all active events for the given tenant.
pub fn dashboard_route(
    _req: &Request<()>,
    dashboard_store: DashboardStore,
    tenant_id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    info!("loading dashboard for {tenant_id}");
    let dashboard = dashboard_store
        .borrow_inner()
        .query_owned(tenant_id.clone())?;

    match dashboard {
        Some(DashboardModel::DashboardData(dashboard)) => {
            info!("dashboard {tenant_id} found");
            let active = dashboard
                .events
                .iter()
                .filter(|e| e.active)
                .cloned()
                .collect();
            let json: Vec<u8> = serde_json::to_vec(&(dashboard + active))?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => {
            warn!("dashboard {tenant_id} not found");
            not_found_route()
        }
    }

    /*let dummy_data = DashboardApi {
      name: "Bucket Golf Leagues".to_string(),
      announcement: "⛳️ New summer leagues of bucket golf just dropped. Rally your crew and start swinging!".to_string(),
      events,
    };*/
}

use crate::not_found_route;
use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

use crate::{verify, DashboardStore};
use log::{info, warn};

/// Fetch the details for a specific event by ID.

pub fn event_details_route(
    req: &Request<()>,
    dashboard_store: DashboardStore,
    id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let auth_ok = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.strip_prefix("Bearer ").unwrap_or(h))
        .and_then(|t| verify(t).ok())
        .is_some();

    if !auth_ok {
        return Ok(
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(b"{}".to_vec())?,
        );
    }

    info!("querying event {id}");
    let event = dashboard_store.borrow_inner().query_owned(id.clone())?;

    match event {
        Some(crate::DashboardModel::Event(event)) => {
            info!("event {id} found");
            let json = serde_json::to_vec(&event)?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => {
            warn!("event {id} not found");
            not_found_route()
        }
    }
}

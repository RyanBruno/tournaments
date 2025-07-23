use crate::{DashboardCommand, DashboardStore};
use http::{Request, Response, StatusCode};
use models::Event;
use std::error::Error;

pub fn create_event_route(
    req: &Request<()>,
    mut dashboard_store: DashboardStore,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let event = Event {
        tenant_id: req.headers().get("x-tenant_id").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string(),
        id: req.headers().get("x-id").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string(),
        name: req.headers().get("x-name").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string(),
        location: req.headers().get("x-location").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string(),
        date: req.headers().get("x-date").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string(),
        image: String::new(),
        banner: None,
        upsell: None,
    };
    dashboard_store.command(&DashboardCommand::CreateEvent(event.clone()))?;
    dashboard_store.fold()?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"created\"}".to_vec())?)
}

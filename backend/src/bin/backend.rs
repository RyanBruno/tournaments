use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

use backend::{
  NetExecutor,
  AsyncTcpListener,
  AsyncHttpRequest,
  event_store,
  Event,
  IndexedStoreHandle,
  EventPatch,
};

use backend::{
  not_found_route,
  error_route,
  dashboard_route,
  event_details_route,
};
fn clear_directory(path: &str) -> io::Result<()> {
  if Path::new(path).exists() {
    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_file() {
        fs::remove_file(path)?;
      } else if path.is_dir() {
        fs::remove_dir_all(path)?;
      }
    }
  }
  Ok(())
}

pub fn seed_example_events(event_store: &IndexedStoreHandle<Event, EventPatch, String>) {
  let examples = vec![
    Event {
      tenant_id: "bucket-golf".into(),
      id: "1".into(),
      name: "Arlington Bucket Golf League".into(),
      location: "Quincy Park, VA".into(),
      date: "Saturday, July 13 – 3:00 PM".into(),
      image: "/static/bucket-golf.jpg".into(),
      banner: Some("⚡ Almost Sold Out".into()),
      upsell: Some("Only 3 slots left".into()),
    },
    Event {
      tenant_id: "bucket-golf".into(),
      id: "2".into(),
      name: "Launch Meetup".into(),
      location: "Liberty Park, DC".into(),
      date: "Monday, July 15 – 5:00 PM".into(),
      image: "/static/launch-meetup.jpg".into(),
      banner: None,
      upsell: None,
    },
    Event {
      tenant_id: "bucket-golf".into(),
      id: "3".into(),
      name: "Weekly Planning Session".into(),
      location: "National Mall, DC".into(),
      date: "Wednesday, July 17 – 12:00 PM".into(),
      image: "/static/planning.jpg".into(),
      banner: Some("🆕 New".into()),
      upsell: Some("Limited spots available".into()),
    },
  ];

  for event in examples {
    if let Err(e) = event_store.create(event.id.clone(), event.clone()) {
      eprintln!("Failed to insert event {:?}: {:?}", event.name, e);
    }
  }

  println!("✅ Example events seeded.");
}


#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn main() -> Result<(), Box<dyn Error>>{
  log4rs::init_file("log4rs.yml", Default::default()).unwrap();

  /* Clear and seed data for development */
  clear_directory("data/events/")?;
  clear_directory("data/snapshots/")?;
  let mut event_store = event_store()?;
  seed_example_events(&event_store);
  event_store.refresh_snapshot()?;

  /* Start the server */
  let executor = NetExecutor::new();
  let listener = AsyncTcpListener::new(8000, executor.clone()).unwrap();
  let server = AsyncHttpRequest::new(listener, executor.clone());

  executor.clone().spawn(async move {
    loop {
      /* Wait for a new request */
      let (request, mut stream) = server.next_request().await.unwrap();

      /* Process the request */
      let response = match request.uri().path() {
        "/dashboard" => dashboard_route(
          &request, event_store.clone(),
          request.headers().get("x-tenant_id")
            .and_then(|v| v.to_str().ok()).unwrap_or_default()
            .to_string(),
        ),
        "/event-details" => event_details_route(
          &request, event_store.clone(),
          request.headers().get("x-id")
            .and_then(|v| v.to_str().ok()).unwrap_or_default()
            .to_string()
        ),

        _ => not_found_route(),
      }.unwrap_or_else(|e| {
        error!("Error processing request: {e}");
        error_route()
      });

      /* Write the response back to the client */
      AsyncHttpRequest::write_response(&mut stream, response).await;
    }
  });

  executor.run();
  Ok(())
}
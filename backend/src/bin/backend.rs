use std::error::Error;

use backend::{
  NetExecutor,
  AsyncTcpListener,
  AsyncHttpRequest,
  event_store,
};

use backend::{
  not_found_route,
  error_route,
  dashboard_route,
  event_details_route,
};

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn main() -> Result<(), Box<dyn Error>>{
  log4rs::init_file("log4rs.yml", Default::default()).unwrap();
  let event_store = event_store()?;

  let executor = NetExecutor::new();
  let listener = AsyncTcpListener::new(8000, executor.clone()).unwrap();
  let server = AsyncHttpRequest::new(listener, executor.clone());

  executor.clone().spawn(async move {
    loop {
      /* Wait for a new request */
      let (request, mut stream) = server.next_request().await.unwrap();

      /* Process the request */
      let response = match request.uri().path() {
        /* Screens */
        // Dashboard
        "/dashboard" => dashboard_route(request, event_store.clone()),
        //"/create-event" => create_event_route(request, event_store.clone()),
        "/event-details" => event_details_route(request, event_store.clone()),

        // Event
        "/modify-event" => panic!(),
        "/get-event" => panic!(),
        "/event-register" => panic!(),

        // Webpage
        "/signup" => panic!(),
        "/login" => panic!(),
        "/create-platform" => panic!(),

        _ => not_found_route(request),
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
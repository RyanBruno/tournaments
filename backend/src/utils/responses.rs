use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn serve_route<T>(_: Request<()>, content_type: &str, res: T) -> http::Result<Response<Vec<u8>>>
  where Vec<u8>: From<T>
{
  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", content_type)
    .header("Access-Control-Allow-Origin", "*")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .body(res.into())
}

pub fn not_found_route() -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  Ok(Response::builder()
    .status(StatusCode::NOT_FOUND)
    .header("Content-Type", "text/html")
    .header("Access-Control-Allow-Origin", "*")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .body("404 Not Found".into())?)
}

pub fn error_route() -> Response<Vec<u8>> {
  match Response::builder()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .header("Content-Type", "text/html")
    .body("500 Internal Server Error".into()) {
      Ok(res) => res,
      Err(e) => {
        error!("Error creating error response, giving up {e}");
        panic!("Error creating error response, giving up {e}");
      }
    }
}
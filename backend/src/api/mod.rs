
pub mod dashboard;
pub mod create_event_route;
pub mod event;

use http::Request;
use http::Response;
use http::StatusCode;
use std::fs::read;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn api_route(_req: Request<()>) -> http::Result<Response<Vec<u8>>>
{
  //let file = File::open("data/archive.rkyv")?;
  let archive_bytes = read("data/archive.rkyv").unwrap();

  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/octet-stream")
    .header("Access-Control-Allow-Origin", "*")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .body(archive_bytes)
}

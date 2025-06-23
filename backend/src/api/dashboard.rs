use http::Request;
use http::Response;
use http::StatusCode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  pub id: String,
  pub name: String,
  pub location: String,
  pub date: String,
  pub image: String,
  pub banner: Option<String>,
  pub upsell: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardApi {
  pub announcment: String,
  pub events: Vec<Event>,
}

pub fn dashboard_route(_req: Request<()>) -> http::Result<Response<Vec<u8>>>
{

    let dummy_data = DashboardApi {
    announcment: "⛳️ New summer leagues of bucket golf just dropped. Rally your crew and start swinging!".to_string(),
    events: vec![
      Event {
        id: "1".into(),
        name: "Arlington Bucket Golf League".into(),
        location: "Quincy Park, VA".into(),
        date: "Saturday, July 13 – 3:00 PM".into(),
        image: "/static/bucket-golf.jpg".into(),
        banner: Some("⚡ Almost Sold Out".into()),
        upsell: Some("Only 3 slots left".into()),
      },
      Event {
        id: "2".into(),
        name: "DC Mini Putt-Off".into(),
        location: "The Yards, DC".into(),
        date: "Sunday, July 21 – 1:00 PM".into(),
        image: "/static/bucket-golf.jpg".into(),
        banner: None,
        upsell: None,
      }
    ],
  };

  let json = match serde_json::to_vec(&dummy_data) {
    Ok(data) => data,
    Err(_) => {
      return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(b"failed to serialize dashboard".to_vec());
    }
  };
  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .header("Access-Control-Allow-Origin", "*")
    .header("Access-Control-Allow-Methods", "*")
    .header("Access-Control-Allow-Headers", "*")
    .body(json)
}
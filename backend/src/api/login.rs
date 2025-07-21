use http::Request;
use http::Response;
use http::StatusCode;
use std::error::Error;
use crate::not_found_route;

use crate::{PlatformStore, PlatformModel};
use models::LoginAttempt;

pub fn login_route(
  _req: &Request<()>,
  platform_store: PlatformStore,
  email: String,
  password: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
  let user = platform_store.borrow_inner()
    .query_owned(format!("user-{}", email.clone()))?;

  let login_attempt = LoginAttempt {
    email,
    password,
  };

  match user {
    Some(PlatformModel::User(user)) if user == login_attempt => {
      let json = serde_json::to_vec(&user)?;

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
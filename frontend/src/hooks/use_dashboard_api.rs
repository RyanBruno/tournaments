use dioxus::prelude::*;
use crate::{
  ClientContext,
  ToastContext, ToastKind, ToastMessage
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
  pub tenant_id: String,
  pub id: String,
  pub name: String,
  pub location: String,
  pub date: String,
  pub image: String,
  pub banner: Option<String>,
  pub upsell: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DashboardApi {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
}


pub fn use_dashboard_api(
  mut toast: Signal<ToastContext>,
  client: Signal<ClientContext>,
) -> Resource<Option<DashboardApi>> {
    use_resource(move || async move {
      let result = client().client.clone().get(
          "http://localhost:8000/dashboard",
        ).header(
          "x-tenant_id",
          "bucket-golf",
        ).send().await;

      let parsed = match result {
        Ok(response) => {
          response.json::<DashboardApi>().await
        },
        Err(e) => Err(e),
      };
      
      match parsed {
        Ok(response) => Some(response),
        Err(_e) => {
          toast.write().toast = Some(ToastMessage {
            message: "Failed to fetch /dashboard".to_string(),
            kind: ToastKind::Error,
          });
          None
        }
      }
    })
}
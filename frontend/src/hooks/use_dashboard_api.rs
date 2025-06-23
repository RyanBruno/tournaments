use dioxus::prelude::*;
use reqwest::Response;
use crate::{ToastContext, ToastKind, ToastMessage};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
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
  pub announcment: String,
  pub events: Vec<Event>,
}


pub fn use_dashboard_api(mut toast: Signal<ToastContext>) -> Resource<Option<DashboardApi>> {
    return use_resource(move || async move {
      let result = reqwest::get("http://localhost:8000/dashboard").await;
      let parsed = match result {
        Ok(response) => {
          response.json::<DashboardApi>().await
        },
        Err(e) => Err(e),
      };
      
      match parsed {
        Ok(response) => Some(response),
        Err(e) => {
          toast.write().toast = Some(ToastMessage {
            message: format!("Failed to fetch /dashboard"),
            kind: ToastKind::Error,
          });
          None
        }
      }
    });
}
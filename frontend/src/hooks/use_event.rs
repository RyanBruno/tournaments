use dioxus::prelude::*;
use crate::{ToastContext, ToastKind, ToastMessage};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
  pub tenent_id: String,
  pub id: String,
  pub name: String,
  pub location: String,
  pub date: String,
  pub image: String,
  pub banner: Option<String>,
  pub upsell: Option<String>,
}


pub fn use_event(id: String, mut toast: Signal<ToastContext>) -> Resource<Option<Event>> {
    return use_resource(move || {
      let id = id.clone();
      async move {
        let url = format!("http://localhost:8000/event-details?id={}", id.clone());
        let result = reqwest::get(url).await;
        let parsed = match result {
          Ok(response) => {
            response.json::<Event>().await
          },
          Err(e) => Err(e),
        };
        
        match parsed {
          Ok(response) => Some(response),
          Err(_e) => {
            toast.write().toast = Some(ToastMessage {
              message: format!("Failed to fetch /dashboard"),
              kind: ToastKind::Error,
            });
            None
          }
        }
      }
  });
}
use dioxus::prelude::*;
use crate::{ToastContext, ClientContext, ToastKind, ToastMessage};

use models::{
  Event,
};

pub fn use_event(
  id: String,
  mut toast: Signal<ToastContext>,
  client: Signal<ClientContext>,
) -> Resource<Option<Event>> {
    use_resource(move || {
      let id = id.clone();
      async move {
        let result = client().client.clone().get(
            "http://localhost:8000/event-details",
          ).header(
            "x-id",
            id,
          ).send().await;

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
              message: "Failed to fetch /event-details".to_string(),
              kind: ToastKind::Error,
            });
            None
          }
        }
      }
  })
}
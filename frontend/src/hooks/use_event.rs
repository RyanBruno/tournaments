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
        let ctx = client();
        let mut req = ctx.client.clone().get(
            "http://localhost:8000/dashboard/event",
          ).header(
            "x-id",
            id,
          );
        if let Some(token) = &ctx.token {
            req = req.bearer_auth(token);
        }
        let result = req.send().await;

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
              message: "Failed to fetch /dashboard/event".to_string(),
              kind: ToastKind::Error,
            });
            None
          }
        }
      }
  })
}
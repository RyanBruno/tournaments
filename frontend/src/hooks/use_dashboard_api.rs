use dioxus::prelude::*;
use crate::{
  ClientContext,
  ToastContext, ToastKind, ToastMessage
};
use models::{
  DashboardView,
};



pub fn use_dashboard_api(
  mut toast: Signal<ToastContext>,
  client: Signal<ClientContext>,
) -> Resource<Option<DashboardView>> {
    use_resource(move || async move {
      let ctx = client();
      let mut req = ctx.client.clone().get(
          "http://localhost:8000/dashboard",
        ).header(
          "x-tenant_id",
          "bucket-golf",
        );
      if let Some(token) = &ctx.token {
          req = req.bearer_auth(token);
      }
      let result = req.send().await;

      let parsed = match result {
        Ok(response) => {
          response.json::<DashboardView>().await
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
use dioxus::prelude::*;
use crate::{ClientContext, ToastContext, ToastKind, ToastMessage};
use models::DashboardView;

pub fn use_platform_name(
    mut toast: Signal<ToastContext>,
    client: Signal<ClientContext>,
) -> Resource<Option<String>> {
    use_resource(move || async move {
        let ctx = client();
        let mut req = ctx
            .client
            .get("http://localhost:8000/dashboard")
            .header("x-tenant_id", "bucket-golf");
        if let Some(token) = &ctx.token {
            req = req.bearer_auth(token);
        }
        let result = req.send().await;
        let parsed = match result {
            Ok(resp) => resp.json::<DashboardView>().await,
            Err(e) => Err(e),
        };
        match parsed {
            Ok(view) => Some(view.name),
            Err(_) => {
                toast.write().toast = Some(ToastMessage {
                    message: "Failed to fetch /dashboard".to_string(),
                    kind: ToastKind::Error,
                });
                None
            }
        }
    })
}

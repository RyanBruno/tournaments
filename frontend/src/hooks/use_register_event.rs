use crate::{ClientContext, ToastContext, ToastKind, ToastMessage};
use dioxus::prelude::*;
use models::Registration;

pub fn use_register_event(
    trigger: Signal<Option<(String, String)>>,
    mut toast: Signal<ToastContext>,
    client: Signal<ClientContext>,
) -> Resource<Option<Registration>> {
    use_resource(move || async move {
        let Some((event_id, email)) = &*trigger.read() else {
            return None;
        };
        let ctx = client();
        let mut req = ctx
            .client
            .post("http://localhost:8000/dashboard/register_event")
            .header("x-id", event_id.clone())
            .header("x-email", email.clone());
        if let Some(token) = &ctx.token {
            req = req.bearer_auth(token);
        }
        let result = req.send().await;
        let parsed = match result {
            Ok(resp) => resp.json::<Registration>().await,
            Err(e) => Err(e),
        };
        match parsed {
            Ok(reg) => Some(reg),
            Err(_) => {
                toast.write().toast = Some(ToastMessage {
                    message: "Failed to register".to_string(),
                    kind: ToastKind::Error,
                });
                None
            }
        }
    })
}

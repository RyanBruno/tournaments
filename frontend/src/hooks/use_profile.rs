use dioxus::prelude::*;
use crate::{ClientContext, ToastContext, ToastKind, ToastMessage};
use models::DashboardUser;

/// Load the profile for the authenticated dashboard user and optionally update the password when triggered.
pub fn use_profile(
    trigger: Signal<Option<String>>, 
    mut toast: Signal<ToastContext>,
    client: Signal<ClientContext>,
) -> Resource<Option<DashboardUser>> {
    // Update when trigger is set
    use_future(move || async move {
        let Some(pass) = &*trigger.read() else { return; };
        let ctx = client();
        let mut req = ctx
            .client
            .patch("http://localhost:8000/dashboard/profile")
            .header("x-password", pass.clone());
        if let Some(token) = &ctx.token {
            req = req.bearer_auth(token);
        }
        let _ = req.send().await;
    });

    // Fetch profile
    use_resource(move || async move {
        let ctx = client();
        let mut req = ctx
            .client
            .get("http://localhost:8000/dashboard/profile");
        if let Some(token) = &ctx.token {
            req = req.bearer_auth(token);
        }
        let result = req.send().await;
        let parsed = match result {
            Ok(resp) => resp.json::<DashboardUser>().await,
            Err(e) => Err(e),
        };
        match parsed {
            Ok(user) => Some(user),
            Err(_) => {
                toast.write().toast = Some(ToastMessage {
                    message: "Failed to fetch /dashboard/profile".to_string(),
                    kind: ToastKind::Error,
                });
                None
            }
        }
    })
}

use crate::{ClientContext, ToastContext, ToastKind, ToastMessage};
use dioxus::prelude::*;
use models::LoginAttempt;

use models::DashboardUser;

pub fn use_dashboard_login(
    login: Signal<Option<LoginAttempt>>,
    mut toast: Signal<ToastContext>,
    client: Signal<ClientContext>,
) -> Resource<Option<DashboardUser>> {
    use_resource(move || async move{
      let login = login.read();
        let login2 = match &*(login) {
            Some(attempt) => attempt,
            _ => return None, // If login is None, we return None immediately
            //None => return None,
        };
        let client = client();
        let mut toast = toast.write();

        let result = client
            .client
            .get("http://localhost:8000/dashboard/login")
            .header("x-email", login2.email.clone())
            .header("x-password", login2.password.clone())
            .send()
            .await;

        let parsed = match result {
            Ok(response) => response.json::<DashboardUser>().await,
            Err(e) => Err(e),
        };

        match parsed {
            Ok(user) => Some(user),
            Err(_) => {
                toast.toast = Some(ToastMessage {
                    message: "Login failed".to_string(),
                    kind: ToastKind::Error,
                });
                None
            }
        }
    })
}

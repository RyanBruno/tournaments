use crate::{ClientContext, ToastContext, ToastKind, ToastMessage};
use dioxus::prelude::*;
use models::LoginAttempt;
use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
}

pub fn use_dashboard_login(
    login: Signal<Option<LoginAttempt>>,
    mut toast: Signal<ToastContext>,
    mut client: Signal<ClientContext>,
) -> Resource<Option<String>> {
    use_resource(move || async move{
      let login = login.read();
        let login2 = match &*(login) {
            Some(attempt) => attempt,
            _ => return None, // If login is None, we return None immediately
            //None => return None,
        };
        let mut client_ctx = client.write();
        let mut toast = toast.write();

        let result = client_ctx
            .client
            .get("http://localhost:8000/dashboard/login")
            .header("x-email", login2.email.clone())
            .header("x-password", login2.password.clone())
            .send()
            .await;

        let parsed = match result {
            Ok(response) => response.json::<TokenResponse>().await,
            Err(e) => Err(e),
        };

        match parsed {
            Ok(token) => {
                client_ctx.set_token(token.token.clone());
                #[cfg(target_arch = "wasm32")]
                if let Some(win) = window() {
                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.set_item("token", &token.token);
                    }
                }
                Some(token.token)
            }
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

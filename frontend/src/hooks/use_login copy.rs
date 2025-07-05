use crate::{ClientContext, ToastContext, ToastKind, ToastMessage};
use dioxus::prelude::*;

use models::User;

pub async fn use_login(
    email: String,
    password: String,
    mut toast: Signal<ToastContext>,
    client: Signal<ClientContext>,
) -> Option<User> {
    let email = email.clone();
    let password = password.clone();
    let result = client()
        .client
        .clone()
        .get("http://localhost:8000/login")
        .header("x-email", email)
        .header("x-password", password)
        .send()
        .await;

    let parsed = match result {
        Ok(response) => response.json::<User>().await,
        Err(e) => Err(e),
    };
    match parsed {
        Ok(response) => Some(response),
        Err(_e) => {
            toast.write().toast = Some(ToastMessage {
                message: "Failed to fetch /login".to_string(),
                kind: ToastKind::Error,
            });
            None
        }
    }
}

use dioxus::prelude::*;
use crate::{ToastContext, ClientContext, ToastKind, ToastMessage};
use models::Bracket;

pub fn use_bracket(id: String, mut toast: Signal<ToastContext>, client: Signal<ClientContext>) -> Resource<Option<Bracket>> {
    use_resource(move || {
        let id = id.clone();
        async move {
            let result = client().client.clone()
                .get("http://localhost:8000/tournament/live")
                .header("x-bracket", id)
                .send()
                .await;

        let parsed = match result {
            Ok(resp) => resp.json::<Bracket>().await,
            Err(e) => Err(e),
        };

        match parsed {
            Ok(bracket) => Some(bracket),
            Err(_) => {
                toast.write().toast = Some(ToastMessage {
                    message: "Failed to fetch /tournament/live".into(),
                    kind: ToastKind::Error,
                });
                None
            }
        }
        }
    })
}


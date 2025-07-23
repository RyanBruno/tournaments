use dioxus::prelude::*;
use crate::{use_bracket, ToastContext, ClientContext};

#[component]
pub fn LiveScores(id: String) -> Element {
    let bracket = use_bracket(
        id.clone(),
        use_context::<Signal<ToastContext>>(),
        use_context::<Signal<ClientContext>>(),
    );


    match bracket.read_unchecked().as_ref() {
        Some(Some(br)) => rsx!(
            div { class: "p-4",
                h1 { class: "text-xl font-bold", "Live Scores" }
                ul {
                    for game in br.matches.iter() {
                        li { "{game.id}: {game.score_a} - {game.score_b}" }
                    }
                }
            }
        ),
        _ => rsx!(),
    }
}

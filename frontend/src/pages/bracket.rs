use dioxus::prelude::*;
use crate::{use_bracket, ToastContext, ClientContext};

#[component]
pub fn BracketPage(id: String) -> Element {
    let bracket_res = use_bracket(
        id,
        use_context::<Signal<ToastContext>>(),
        use_context::<Signal<ClientContext>>(),
    );
    match bracket_res.read_unchecked().as_ref() {
        Some(Some(bracket)) => rsx!(
            div { class: "p-4",
                h1 { class: "text-2xl font-bold", "{bracket.name}" }
                ul {
                    for game in bracket.matches.iter() {
                        li { "{game.participant_a} vs {game.participant_b}: {game.score_a} - {game.score_b}" }
                    }
                }
            }
        ),
        _ => rsx!(),
    }
}

use dioxus::prelude::*;
use frontend::{ToastContext, ClientContext, Route, Toast};
use reqwest::Client;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");
//const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy)]
pub struct AppContext {
    pub splash: bool,
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppContext {
        splash: true,
    }));

    use_context_provider(|| Signal::new(ToastContext {
        toast: None,
    }));

    use_context_provider(|| Signal::new(ClientContext {
        client: Client::new(),
    }));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        //document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
        Toast {}
    }
}
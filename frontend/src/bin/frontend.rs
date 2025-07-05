use dioxus::prelude::*;
use frontend::{BrandContext, ClientContext, Route, Toast, ToastContext};
use reqwest::Client;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const LOGO_SVG: Asset = asset!("/assets/logo.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(ToastContext {
        toast: None,
    }));

    use_context_provider(|| Signal::new(ClientContext {
        client: Client::new(),
    }));

    use_context_provider(|| Signal::new(BrandContext {
      name: "bracketly™".to_string(),
      logo: LOGO_SVG,
      
      primary_color: "#1e2a38ff".to_string(),
      secondary_color: "#1181c2ff".to_string(),
    }));

    rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: MAIN_CSS }
      //document::Link { rel: "stylesheet", href: TAILWIND_CSS }
      Router::<Route> {}
      Toast {}
    }
}
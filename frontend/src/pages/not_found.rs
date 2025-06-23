use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
  rsx! {
    div { style: "min-height: 100vh; background-color: #f9fafb; display: flex; align-items: center; justify-content: center; font-family: sans-serif;",
      div { style: "text-align: center; max-width: 28rem; padding: 2rem;",
        h1 { style: "font-size: 3rem; font-weight: bold; color: #EA3E4E; margin-bottom: 1rem;", "404" }
        h2 { style: "font-size: 1.5rem; font-weight: 600; color: #111827; margin-bottom: 1rem;", "Page not found" }
        p { style: "color: #6b7280; font-size: 1rem; margin-bottom: 2rem;",
          "We couldn't find the page you were looking for."
        }
        pre { style: "color: #dc2626; font-size: 0.875rem; background-color: #fef2f2; padding: 0.75rem; border-radius: 0.375rem; overflow-x: auto;",
          "log:\nattempted to navigate to: {route:?}"
        }
        Link {
          to: Route::Homepage {},
          style: "display: inline-block; margin-top: 2rem; background-color: #EA3E4E; color: white; font-weight: 600; padding: 0.75rem 1.5rem; border-radius: 0.5rem; text-decoration: none;",
          "Back to Home"
        }
      }
    }
  }
}

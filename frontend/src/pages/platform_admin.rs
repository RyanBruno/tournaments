use dioxus::prelude::*;
use crate::{BrandContext, Route};

#[component]
pub fn PlatformAdmin() -> Element {
    let brand = use_context::<Signal<BrandContext>>();
    let BrandContext { name, logo: _, primary_color: _, secondary_color: _} = brand.read().clone();

    rsx! {
      div { style: "min-height: 100vh; padding: 3rem 1rem; background-color: #f9fafb; font-family: sans-serif;",
        div { style: "max-width: 40rem; margin: 0 auto; text-align: center;",
          h1 { style: "font-size: 2rem; font-weight: bold; color: #111827; margin-bottom: 1rem;",
            "{name} Admin"
          }
          p { style: "font-size: 1rem; color: #4b5563; margin-bottom: 2rem;",
            "Manage your community settings and events."
          }
          div { style: "display: flex; flex-direction: column; gap: 1rem;",
            Link { to: Route::ManagePlatform {},
              button { style: button_style(), "Manage Platform" }
            }
            Link { to: Route::Dashboard {},
              button { style: button_style(), "View Dashboard" }
            }
          }
        }
      }
    }
}

fn button_style() -> &'static str {
    "background-color: #4f46e5; color: white; font-weight: 600; padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; cursor: pointer;"
}

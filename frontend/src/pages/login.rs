use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Login() -> Element {
    rsx!(
        div { style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #f9fafb; font-family: sans-serif;",
            div { style: "width: 100%; max-width: 24rem; background: white; padding: 2rem; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
                h2 { style: "text-align: center; font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; color: #111827;", "Sign in to Konvo" },
                form { style: "display: flex; flex-direction: column; gap: 1rem;",
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Email" }
                        input { r#type: "email", placeholder: "you@example.com", style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;" }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Password" }
                        input { r#type: "password", placeholder: "••••••••", style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;" }
                    }
                    button { r#type: "submit", style: "margin-top: 1rem; background-color: #EA3E4E; color: white; font-weight: 600; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer;", "Sign In" }
                }
                Link {
                  to: Route::GetStarted {},
                  p { style: "margin-top: 1rem; text-align: center; font-size: 0.875rem; color: #6b7280;",
                      "Don't have an account? "
                      a { href: "#", style: "color: #EA3E4E; text-decoration: none; font-weight: 500;", "Sign up" }
                  }
                }
            }
        }
    )
}
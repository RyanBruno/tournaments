use dioxus::prelude::*;
use crate::{Route, BrandContext, ClientContext};
use models::PlatformUser;

#[component]
pub fn Signup() -> Element {
  let brand = use_context::<Signal<BrandContext>>();
  let client = use_context::<Signal<ClientContext>>();
  let BrandContext {name: _, logo: _, primary_color: _, secondary_color} = brand.read().clone();
  let mut email = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let mut confirm = use_signal(|| String::new());
  let mut submit = use_signal(|| false);

  use_future(move || async move {
    if !submit() { return; }
    let user = PlatformUser { email: email(), password: password() };
    let _ = client().client
      .post("http://localhost:8000/platform/user")
      .json(&user)
      .send()
      .await;
  });
  rsx!(
    div { style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #f9fafb; font-family: sans-serif;",
      div { style: "width: 100%; max-width: 24rem; background: white; padding: 2rem; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
        h2 { style: "text-align: center; font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; color: #111827;",
          "Create your account"
        }
        form { style: "display: flex; flex-direction: column; gap: 1rem;",
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;",
              "Email"
            }
            input {
              r#type: "email",
              placeholder: "you@example.com",
              oninput: move |e| email.set(e.value()),
              value: email.clone(),
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
            }
          }
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;",
              "Password"
            }
            input {
              r#type: "password",
              placeholder: "••••••••",
              oninput: move |e| password.set(e.value()),
              value: password.clone(),
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
            }
          }
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;",
              "Confirm Password"
            }
            input {
              r#type: "password",
              placeholder: "••••••••",
              oninput: move |e| confirm.set(e.value()),
              value: confirm.clone(),
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
            }
          }
          Link { to: Route::ConfigurePlatform {},
            button {
              r#type: "submit",
              onclick: move |_| submit.set(true),
              style: "margin-top: 1rem; background-color: {secondary_color}; color: white; font-weight: 600; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer;",
              "Sign Up"
            }
          }
        }
        Link { to: Route::Login {},
          p { style: "margin-top: 1rem; text-align: center; font-size: 0.875rem; color: #6b7280;",
            "Already have an account? "
            a {
              href: "#",
              style: "color: {secondary_color}; text-decoration: none; font-weight: 500;",
              "Sign in"
            }
          }
        }
      }
    }
  )
}

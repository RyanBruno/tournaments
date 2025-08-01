use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::{Route, BrandContext, use_platform_login, ClientContext, ToastContext};
use models::{LoginAttempt};

#[component]
pub fn Login() -> Element {
  let brand = use_context::<Signal<BrandContext>>();
  let BrandContext {name, logo: _, primary_color: _, secondary_color} = brand.read().clone();
  let mut email = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let mut login_attempt = use_signal(|| None);


  let user = use_platform_login(
    login_attempt,
    use_context::<Signal<ToastContext>>(),
    use_context::<Signal<ClientContext>>(),
  );
  println!("User: {:?}", user.read());

  let navigator = use_navigator();
  use_effect(move || {
    match *user.read() {
      Some(Some(_)) => {navigator.push(Route::ManagePlatform {});},
      _ => {},
    }
  });

  rsx!(
    div { style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #f9fafb; font-family: sans-serif;",
      div { style: "width: 100%; max-width: 24rem; background: white; padding: 2rem; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
        h2 { style: "text-align: center; font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; color: #111827;",
          "Sign in to {name}"
        }
        div { style: "display: flex; flex-direction: column; gap: 1rem;",
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;",
              "Email"
            }
            input {
              r#type: "email",
              placeholder: "you@example.com",
              oninput: move |e| {
                  email.set(e.value());
              },
              value: email,
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
              oninput: move |e| {
                  password.set(e.value());
              },
              value: password,
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
            }
          }
          button {
            r#type: "submit",
            onclick: move |_| {
                login_attempt
                    .set(
                        Some(LoginAttempt {
                            email: email(),
                            password: password(),
                        }),
                    );
            },
            style: "margin-top: 1rem; background-color: {secondary_color}; color: white; font-weight: 600; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer;",
            "Sign In"
          }
        }
        Link { to: Route::GetStarted {},
          p { style: "margin-top: 1rem; text-align: center; font-size: 0.875rem; color: #6b7280;",
            "Don't have an account? "
            a {
              href: "#",
              style: "color: {secondary_color}; text-decoration: none; font-weight: 500;",
              "Sign up"
            }
          }
        }
      }
    }
  )
}
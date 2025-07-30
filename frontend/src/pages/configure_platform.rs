use dioxus::prelude::*;
use crate::{BrandContext, ClientContext};
use models::Platform;

#[component]
pub fn ConfigurePlatform() -> Element {
    let brand = use_context::<Signal<BrandContext>>();
    let client = use_context::<Signal<ClientContext>>();
    let BrandContext { name: _, logo: _, primary_color: _, secondary_color } = brand.read().clone();

    let mut community_name = use_signal(|| String::new());
    let mut community_description = use_signal(|| String::new());
    let mut platform_url = use_signal(|| String::new());
    let mut submit = use_signal(|| false);

    use_future(move || async move {
        if !submit() { return; }
        let platform = Platform {
            tenant_id: "bucket-golf".into(),
            community_name: community_name(),
            community_description: community_description(),
            platform_url: platform_url(),
        };
        let _ = client().client
            .post("http://localhost:8000/platform/create")
            .json(&platform)
            .send()
            .await;
    });

    rsx!(
        div { style: "min-height: 100vh; padding: 3rem 1rem; background-color: #f9fafb; font-family: sans-serif;",
            div { style: "max-width: 40rem; margin: 0 auto;",
                h1 { style: "font-size: 2rem; font-weight: bold; color: #111827; text-align: center; margin-bottom: 2rem;",
                    "Configure Your Platform"
                }
                p { style: "font-size: 1rem; color: #4b5563; margin-bottom: 1.5rem;",
                    "Set your branding, community rules, and permissions (you can change this later)."
                }
                form { style: "display: flex; flex-direction: column; gap: 1rem;",
                    input {
                        r#type: "text",
                        placeholder: "Community Name",
                        oninput: move |e| community_name.set(e.value()),
                        value: community_name.clone(),
                        style: "padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                    }
                    textarea {
                        placeholder: "Community Description",
                        oninput: move |e| community_description.set(e.value()),
                        value: community_description.clone(),
                        style: "padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; height: 100px;",
                    }
                    input {
                        r#type: "url",
                        placeholder: "Platform URL",
                        oninput: move |e| platform_url.set(e.value()),
                        value: platform_url.clone(),
                        style: "padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                    }
                    p { style: "font-size: 0.875rem; color: #6b7280; margin-top: -0.75rem; margin-bottom: 0.5rem;",
                        "DNS update instructions will be sent to your account's email"
                    }
                    button { style: "background-color: {secondary_color}; color: white; font-weight: 600; padding: 0.75rem 1.5rem; border-radius: 0.5rem; border: none; cursor: pointer;",
                        onclick: move |_| submit.set(true),
                        "Save & Continue"
                    }
                }
            }
        }
    )
}

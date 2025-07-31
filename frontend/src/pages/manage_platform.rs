use dioxus::prelude::*;
use crate::{
    BrandContext, ClientContext, ToastContext, ToastKind, ToastMessage,
};
use models::{Platform, PlatformPatch};

#[component]
pub fn ManagePlatform() -> Element {
    let brand = use_context::<Signal<BrandContext>>();
    let client = use_context::<Signal<ClientContext>>();
    let BrandContext { name: _, logo: _, primary_color: _, secondary_color } = brand.read().clone();

    let mut community_name = use_signal(|| String::new());
    let mut community_description = use_signal(|| String::new());
    let mut platform_url = use_signal(|| String::new());
    let mut submit = use_signal(|| false);
    let mut loaded = use_signal(|| false);
    let mut toast = use_context::<Signal<ToastContext>>();

    use_future(move || async move {
        if loaded() {
            return;
        }
        let ctx = client();
        let mut req = ctx
            .client
            .get("http://localhost:8000/platform/info")
            .header("x-tenant_id", "bucket-golf");
        if let Some(token) = &ctx.token {
            req = req.bearer_auth(token);
        }
        if let Ok(resp) = req.send().await {
            if let Ok(p) = resp.json::<Platform>().await {
                community_name.set(p.community_name);
                community_description.set(p.community_description);
                platform_url.set(p.platform_url);
            }
        }
        loaded.set(true);
    });

    use_future(move || async move {
        if !submit() {
            return;
        }
        let patch = PlatformPatch {
            tenant_id: "bucket-golf".into(),
            community_name: Some(community_name()),
            community_description: Some(community_description()),
            platform_url: Some(platform_url()),
        };
        let result = client()
            .client
            .post("http://localhost:8000/platform/update")
            .json(&patch)
            .send()
            .await;

        let mut toast = toast.write();
        match result {
            Ok(resp) if resp.status().is_success() => {
                toast.toast = Some(ToastMessage {
                    message: "Platform updated".to_string(),
                    kind: ToastKind::Success,
                });
            }
            _ => {
                toast.toast = Some(ToastMessage {
                    message: "Failed to update platform".to_string(),
                    kind: ToastKind::Error,
                });
            }
        }
        submit.set(false);
    });

    rsx!(
        div { style: "min-height: 100vh; display: flex; font-family: sans-serif; background-color: #f9fafb;",
            aside { style: "width: 14rem; background-color: #111827; color: white; padding: 2rem;",
                h2 { style: "font-size: 1.25rem; font-weight: 600; margin-bottom: 1.5rem;", "Admin" }
                nav {
                    ul { style: "display: flex; flex-direction: column; gap: 1rem; list-style: none; padding: 0;",
                        li { "Platform" }
                        li { "Users" }
                        li { "Events" }
                    }
                }
            }
            main { style: "flex: 1; display: flex; flex-direction: column;",
                header { style: "background: white; padding: 1rem 2rem; box-shadow: 0 1px 4px rgba(0,0,0,0.05);",
                    h1 { style: "font-size: 1.5rem; font-weight: 600;", "Manage Platform" }
                }
                section { style: "flex: 1; padding: 2rem; max-width: 40rem;",
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
                        button { style: "background-color: {secondary_color}; color: white; font-weight: 600; padding: 0.75rem 1.5rem; border-radius: 0.5rem; border: none; cursor: pointer;",
                            onclick: move |_| submit.set(true),
                            "Update Platform"
                        }
                    }
                }
            }
        }
    )
}

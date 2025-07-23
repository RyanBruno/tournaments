use crate::{BrandContext, ClientContext, ToastContext, ToastKind, ToastMessage};
use dioxus::prelude::*;
use models::Event;
use std::time::SystemTime;

#[component]
pub fn CreateEvent() -> Element {
    let brand = use_context::<Signal<BrandContext>>();
    let client = use_context::<Signal<ClientContext>>();
    let mut toast = use_context::<Signal<ToastContext>>();
    let BrandContext { name: _, logo: _, primary_color: _, secondary_color } = brand.read().clone();
    let mut name = use_signal(|| String::new());
    let mut location = use_signal(|| String::new());
    let mut address = use_signal(|| String::new());
    let mut datetime = use_signal(|| String::new());
    let mut sport = use_signal(|| String::new());
    let mut new_event = use_signal(|| None as Option<Event>);

    let _ = use_resource(move || async move {
        let Some(event) = new_event.read().clone() else { return None };
        let result = client()
            .client
            .get("http://localhost:8000/create-event")
            .header("x-tenant_id", event.tenant_id.clone())
            .header("x-id", event.id.clone())
            .header("x-name", event.name.clone())
            .header("x-location", event.location.clone())
            .header("x-date", event.date.clone())
            .send()
            .await;

        match result {
            Ok(_) => Some(()),
            Err(_) => {
                let mut t = toast.write();
                t.toast = Some(ToastMessage {
                    message: "Failed to create event".to_string(),
                    kind: ToastKind::Error,
                });
                None
            }
        }
    });

    rsx!(
        div { style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #f9fafb; font-family: sans-serif;",
            div { style: "
                width: 100%;
                max-width: 28rem;
                background: white;
                padding: 2rem;
                border-radius: 0.5rem;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            ",
                h2 { style: "text-align: center; font-size: 1.5rem; font-weight: bold; margin-bottom: 1.5rem; color: #111827;", "Create an Event" }
                form { style: "display: flex; flex-direction: column; gap: 1rem;",
                    onsubmit: move |e| {
                        e.prevent_default();
                        new_event.set(Some(Event {
                            tenant_id: "bucket-golf".to_string(),
                            id: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string(),
                            name: name(),
                            location: location(),
                            date: datetime(),
                            image: String::new(),
                            banner: None,
                            upsell: None,
                        }));
                    },
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Event Name" }
                        input {
                            r#type: "text",
                            placeholder: "e.g. Saturday Basketball",
                            oninput: move |e| { name.set(e.value()); },
                            value: name,
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                        }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Event Location" }
                        input {
                            r#type: "text",
                            placeholder: "Central Park, NYC",
                            oninput: move |e| { location.set(e.value()); },
                            value: location,
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                        }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Event Address" }
                        input {
                            r#type: "text",
                            placeholder: "79th Street & 85th Street, New York, NY 10024",
                            oninput: move |e| { address.set(e.value()); },
                            value: address,
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                        }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Date & Time" }
                        input {
                            r#type: "datetime-local",
                            oninput: move |e| { datetime.set(e.value()); },
                            value: datetime,
                            style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;",
                        }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Sport" }
                        select { style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; color: #374151;",
                            oninput: move |e| { sport.set(e.value()); },
                            value: sport,
                            option { value: "", disabled: true, selected: true, "Select a sport" }
                            option { value: "basketball", "Basketball" }
                            option { value: "soccer", "Soccer" }
                            option { value: "lifting", "Lifting" }
                            option { value: "running", "Running" }
                            option { value: "yoga", "Yoga" }
                        }
                    }
                    button {
                        r#type: "submit",
                        style: "
                            margin-top: 1rem;
                            background-color: {secondary_color};
                            color: white;
                            font-weight: 600;
                            padding: 0.75rem;
                            border: none;
                            border-radius: 0.5rem;
                            cursor: pointer;
                        ",
                        "Create Event"
                    }
                }
            }
        }
    )
}


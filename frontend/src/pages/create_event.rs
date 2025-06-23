use dioxus::prelude::*;

#[component]
pub fn CreateEvent() -> Element {
  rsx!(
    div {
      style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; background-color: #f9fafb; font-family: sans-serif;",
      div {
        style: "
          width: 100%;
          max-width: 28rem;
          background: white;
          padding: 2rem;
          border-radius: 0.5rem;
          box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        ",
        h2 {
          style: "text-align: center; font-size: 1.5rem; font-weight: bold; margin-bottom: 1.5rem; color: #111827;",
          "Create an Event"
        }
        form {
          style: "display: flex; flex-direction: column; gap: 1rem;",
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Event Name" }
            input {
              r#type: "text",
              placeholder: "e.g. Saturday Basketball",
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;"
            }
          }
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Event Location" }
            input {
              r#type: "text",
              placeholder: "Central Park, NYC",
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;"
            }
          }
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Event Address" }
            input {
              r#type: "text",
              placeholder: "79th Street & 85th Street, New York, NY 10024",
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;"
            }
          }
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Date & Time" }
            input {
              r#type: "datetime-local",
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem;"
            }
          }
          div {
            label { style: "display: block; margin-bottom: 0.25rem; font-weight: 500;", "Sport" }
            select {
              style: "width: 100%; padding: 0.75rem; border: 1px solid #d1d5db; border-radius: 0.375rem; color: #374151;",
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
              background-color: #EA3E4E;
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

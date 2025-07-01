use dioxus::prelude::*;

#[component]
pub fn ProfilePage() -> Element {
  rsx!(
    div {
      style: "
        min-height: 100vh;
        background-color: #f9fafb;
        padding: 3rem 1rem;
        font-family: sans-serif;
        display: flex;
        justify-content: center;
      ",
      div {
        style: "
          background: white;
          max-width: 48rem;
          width: 100%;
          padding: 2rem;
          border-radius: 1rem;
          box-shadow: 0 4px 12px rgba(0,0,0,0.05);
        ",
        // Header
        h1 {
          style: "font-size: 2rem; font-weight: bold; color: #111827; margin-bottom: 2rem;",
          "Your Profile"
        }

        // Profile Info
        div {
          style: "display: flex; align-items: center; gap: 1.5rem; margin-bottom: 2rem;",
          img {
            src: "/static/profile-placeholder.png", // replace w/ actual user profile pic
            alt: "Profile Picture",
            style: "width: 5rem; height: 5rem; border-radius: 9999px; object-fit: cover; border: 2px solid #e5e7eb;"
          }
          div {
            h2 {
              style: "font-size: 1.25rem; font-weight: 600; color: #111827;",
              "Jane Doe"
            }
            p {
              style: "color: #6b7280; font-size: 0.875rem;",
              "janedoe@example.com"
            }
          }
        }

        // Editable Form
        form {
          style: "display: flex; flex-direction: column; gap: 1rem;",
          input {
            r#type: "text",
            placeholder: "Full Name",
            value: "Jane Doe",
            style: base_input()
          }
          input {
            r#type: "email",
            placeholder: "Email Address",
            value: "janedoe@example.com",
            style: base_input()
          }
          input {
            r#type: "password",
            placeholder: "Change Password",
            style: base_input()
          }
          button {
            style: "
              margin-top: 1rem;
              background-color: #4f46e5;
              color: white;
              font-weight: 600;
              font-size: 1rem;
              padding: 0.75rem;
              border: none;
              border-radius: 0.5rem;
              cursor: pointer;
            ",
            "Update Profile"
          }
        }
      }
    }
  )
}

// Utility style helper
fn base_input() -> &'static str {
  "
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.95rem;
  "
}

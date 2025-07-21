use dioxus::prelude::*;
use crate::{Route, BrandContext};

#[component]
pub fn GetStarted() -> Element {
  let brand = use_context::<Signal<BrandContext>>();
  let BrandContext {name, logo: _, primary_color: _, secondary_color} = brand.read().clone();

    rsx!(
      div { style: "min-height: 100vh; background-color: #ffffff; font-family: sans-serif; padding: 3rem 1rem;",
        div { style: "max-width: 40rem; margin: 0 auto;",
          h1 { style: "font-size: 2rem; font-weight: bold; color: #111827; text-align: center; margin-bottom: 2rem;",
            "Get Started with {name}"
          }
          p { style: "font-size: 1rem; color: #4b5563; margin-bottom: 1.5rem;",
            "Follow these quick steps to start building your community today."
          }
          ol { style: "list-style: decimal; padding-left: 1.5rem; color: #374151;",
            li { style: "margin-bottom: 1rem;", "Create your account by signing up." }
            li { style: "margin-bottom: 1rem;", "Configure your community platform." }
            li { style: "margin-bottom: 1rem;",
              "Track engagement and grow using our insights dashboard."
            }
          }
          Link { to: Route::Signup {},
            div { style: "text-align: center; margin-top: 2rem;",
              button { style: "background-color: {secondary_color}; color: white; font-weight: 600; padding: 0.75rem 1.5rem; border-radius: 0.5rem; border: none; cursor: pointer;",
                "Start Now"
              }
            }
          }
        }
      }
    )
}


#[component]
pub fn InsightsDashboard() -> Element {
  let brand = use_context::<Signal<BrandContext>>();
  let BrandContext {name: _, logo: _, primary_color: _, secondary_color} = brand.read().clone();
    rsx!(
      div { style: "min-height: 100vh; padding: 3rem 1rem; background-color: #ffffff; font-family: sans-serif;",
        div { style: "max-width: 48rem; margin: 0 auto;",
          h1 { style: "font-size: 2rem; font-weight: bold; color: #111827; text-align: center; margin-bottom: 2rem;",
            "Community Insights Dashboard"
          }
          p { style: "font-size: 1rem; color: #4b5563; margin-bottom: 2rem; text-align: center;",
            "Visualize member growth, event participation, and overall engagement trends."
          }
          div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 2rem; text-align: center;",
            div { style: "padding: 1rem; border: 1px solid #e5e7eb; border-radius: 0.5rem; background: #f3f4f6;",
              h3 { style: "font-size: 1.25rem; font-weight: 600;", "Members" }
              p { style: "font-size: 2rem; color: {secondary_color};",
                "1,204"
              }
            }
            div { style: "padding: 1rem; border: 1px solid #e5e7eb; border-radius: 0.5rem; background: #f3f4f6;",
              h3 { style: "font-size: 1.25rem; font-weight: 600;", "Events Run" }
              p { style: "font-size: 2rem; color: {secondary_color};",
                "78"
              }
            }
            div { style: "padding: 1rem; border: 1px solid #e5e7eb; border-radius: 0.5rem; background: #f3f4f6;",
              h3 { style: "font-size: 1.25rem; font-weight: 600;", "Weekly Engagement" }
              p { style: "font-size: 2rem; color: #EA3E4E;", "64%" }
            }
          }
        }
      }
    )
}

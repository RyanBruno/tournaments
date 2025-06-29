use dioxus::prelude::*;

pub mod hooks;
pub use hooks::use_dashboard_api::use_dashboard_api;
pub use hooks::use_dashboard_api::DashboardApi;
pub use hooks::use_dashboard_api::Event;
pub use hooks::use_event::use_event;

pub mod components;
pub use components::toast::Toast;

pub mod context;
pub use context::toast::ToastContext;
pub use context::toast::ToastKind;
pub use context::toast::Toast as ToastMessage;
pub use context::client::ClientContext;

pub mod pages;
pub use pages::homepage::Homepage;
pub use pages::login::Login;
pub use pages::signup::Signup;
pub use pages::get_started::GetStarted;
pub use pages::get_started::ConfigurePlatform;
pub use pages::not_found::PageNotFound;
pub use pages::dashboard::Dashboard;
pub use pages::create_event::CreateEvent;
pub use pages::event_details::EventDetails;
pub use pages::event_register::RegisterEvent;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Homepage,
    #[route("/login")]
    Login,
    #[route("/signup")]
    Signup,
    #[route("/get-started")]
    GetStarted,
    #[route("/configure-platform")]
    ConfigurePlatform,
    #[route("/dashboard")]
    Dashboard,
    #[route("/create-event")]
    CreateEvent,
    #[route("/event/:id")]
    EventDetails { id: String },
    #[route("/event-register/:id")]
    RegisterEvent { id: String },
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
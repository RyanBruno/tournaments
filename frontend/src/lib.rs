use dioxus::prelude::*;

pub mod hooks;
pub use hooks::use_dashboard_api::use_dashboard_api;
pub use hooks::use_event::use_event;
pub use hooks::use_platform_login::use_platform_login;
pub use hooks::use_register_event::use_register_event;

pub mod components;
pub use components::notifications::NotificationsDropdown;
pub use components::toast::Toast;

pub mod context;
pub use context::brand::BrandContext;
pub use context::client::ClientContext;
pub use context::toast::Toast as ToastMessage;
pub use context::toast::ToastContext;
pub use context::toast::ToastKind;

pub mod pages;
pub use pages::create_event::CreateEvent;
pub use pages::dashboard::Dashboard;
pub use pages::event_details::EventDetails;
pub use pages::event_register::RegisterEvent;
pub use pages::get_started::ConfigurePlatform;
pub use pages::get_started::GetStarted;
pub use pages::homepage::Homepage;
pub use pages::login::Login;
pub use pages::not_found::PageNotFound;
pub use pages::profile::ProfilePage;
pub use pages::signup::Signup;

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
    #[route("/profile")]
    ProfilePage,
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

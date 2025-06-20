use dioxus::prelude::*;

pub mod pages;
pub use pages::homepage::Homepage;
pub use pages::login::Login;
pub use pages::signup::Signup;
pub use pages::get_started::GetStarted;
pub use pages::get_started::ConfigurePlatform;
pub use pages::not_found::PageNotFound;
pub use pages::dashboard::Dashboard;
pub use pages::create_event::CreateEvent;

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
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
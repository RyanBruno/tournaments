
mod platform;
pub use platform::{Platform, PlatformPatch, User, UserPatch, LoginAttempt};

mod dashboard;
pub use dashboard::DashboardView;
pub use dashboard::{Event, EventPatch, ArchivedEvent};

mod utils;
pub use utils::{EntityId, Patch};
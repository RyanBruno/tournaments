
mod platform;
pub use platform::{Platform, PlatformPatch, PlatformUser, PlatformUserPatch, LoginAttempt};

mod dashboard;
pub use dashboard::{DashboardData, DashboardView, DashboardUser, DashboardUserPatch};
pub use dashboard::{Event, EventPatch, ArchivedEvent};

mod utils;
pub use utils::{EntityId, Patch};
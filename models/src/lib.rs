
mod platform;
pub use platform::user::{PlatformUser, PlatformUserPatch, LoginAttempt};
pub use platform::platform::{Platform, PlatformPatch};

mod dashboard;
pub use dashboard::store::{DashboardData, DashboardView};
pub use dashboard::event::{Event, EventPatch, ArchivedEvent};
pub use dashboard::user::{DashboardUser, DashboardUserPatch};

mod utils;
pub use utils::{EntityId, Patch};
#[cfg(not(target_arch = "wasm32"))]
pub use utils::{hash_password, verify_password};
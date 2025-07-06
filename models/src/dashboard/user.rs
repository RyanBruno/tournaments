use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};
use rkyv::{
    Archive,
    Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};

use crate::Patch;

#[cfg(not(target_arch = "wasm32"))]
use crate::{verify_password, hash_password};

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct DashboardUser {
  pub email: String,
  #[serde(skip)]
  pub password: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl DashboardUser {
  pub fn new(email: String, password: String) -> Self {
    Self {
      email,
      password: hash_password(&password),
    }
  }
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct DashboardUserPatch {
  pub password: Option<String>,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct LoginAttempt {
  pub email: String,
  pub password: String,
}


#[cfg(not(target_arch = "wasm32"))]
impl PartialEq<LoginAttempt> for DashboardUser {
  fn eq(&self, attempt: &LoginAttempt) -> bool {
    if self.email != attempt.email {
      return false;
    }

    verify_password(&attempt.password, &self.password)
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl Patch<DashboardUser> for DashboardUserPatch {
  fn apply_to(self, target: &mut DashboardUser) -> () {
    if let Some(password) = self.password {
      target.password = hash_password(&password);
    }
  }
}

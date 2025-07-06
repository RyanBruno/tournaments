use std::ops::Add;

use serde::{Deserialize, Serialize};
use rkyv::{
    Archive,
    Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};
use crate::Patch;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, PartialEq)]
pub struct DashboardData {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
}

impl Add<Vec<Event>> for DashboardData {
  type Output = DashboardView;
  fn add(self, rhs: Vec<Event>) -> DashboardView {
    DashboardView { announcement: self.announcement, name: self.name, events: self.events, active_events: rhs }
  }
}

#[derive(Archive, Serialize, Deserialize, Clone, RkyvDeserialize, RkyvSerialize, PartialEq)]
pub struct DashboardView {
  pub announcement: String,
  pub name: String,
  pub events: Vec<Event>,
  pub active_events: Vec<Event>,
}
#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Event {
  pub tenant_id: String,
  pub id: String,
  pub name: String,
  pub location: String,
  pub date: String,
  pub image: String,
  pub banner: Option<String>,
  pub upsell: Option<String>,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct EventPatch {
  pub id: Option<String>,
  pub name: Option<String>,
  pub location: Option<String>,
  pub date: Option<String>,
  pub image: Option<String>,
  pub banner: Option<Option<String>>,
  pub upsell: Option<Option<String>>,
}

impl Patch<Event> for EventPatch {
  fn apply_to(self, target: &mut Event) {
    if let Some(id) = self.id {
      target.id = id;
    }
    if let Some(name) = self.name {
      target.name = name;
    }
    if let Some(location) = self.location {
      target.location = location;
    }
    if let Some(date) = self.date {
      target.date = date;
    }
    if let Some(image) = self.image {
      target.image = image;
    }
    if let Some(banner) = self.banner {
      target.banner = banner;
    }
    if let Some(upsell) = self.upsell {
      target.upsell = upsell;
    }
  }
}
/* Users */

#[cfg(not(target_arch = "wasm32"))]
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
#[cfg(not(target_arch = "wasm32"))]
use argon2::{PasswordHash, PasswordVerifier};

#[derive(Default, Clone, Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug)]
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

#[derive(Default, Clone, Archive, Serialize, Deserialize, RkyvSerialize, RkyvDeserialize)]
pub struct DashboardUserPatch {
  pub password: Option<String>,
}

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

    let parsed_hash = PasswordHash::new(&self.password);
    if let Ok(parsed) = parsed_hash {
      Argon2::default()
        .verify_password(attempt.password.as_bytes(), &parsed)
        .is_ok()
    } else {
      false
    }
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

#[cfg(not(target_arch = "wasm32"))]
fn hash_password(password: &str) -> String {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
  hash
}
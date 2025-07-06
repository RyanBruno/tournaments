use crate::{
  Patch,
  EntityId,
};
use rkyv::{
    Archive, Deserialize, Serialize,
};

#[cfg(not(target_arch = "wasm32"))]
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
#[cfg(not(target_arch = "wasm32"))]
use argon2::{PasswordHash, PasswordVerifier};
use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};

#[derive(Default, Clone, Archive, Serialize, Deserialize, SarSerialize, SarDeserialize, Debug)]
pub struct PlatformUser {
  pub email: String,
  #[serde(skip)]
  pub password: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl PlatformUser {
  pub fn new(email: String, password: String) -> Self {
    Self {
      email,
      password: hash_password(&password),
    }
  }
}

#[derive(Default, Clone, Archive, Serialize, Deserialize, SarSerialize, SarDeserialize)]
pub struct PlatformUserPatch {
  pub password: Option<String>,
}

pub struct LoginAttempt {
  pub email: String,
  pub password: String,
}


#[cfg(not(target_arch = "wasm32"))]
impl PartialEq<LoginAttempt> for PlatformUser {
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
impl Patch<PlatformUser> for PlatformUserPatch {
  fn apply_to(self, target: &mut PlatformUser) -> () {
    if let Some(password) = self.password {
      target.password = hash_password(&password);
    }
  }
}


#[derive(Default, Clone, Archive, Serialize, Deserialize, SarSerialize, SarDeserialize)]
pub struct Platform {
  pub tenant_id: EntityId,
  pub community_name: String,
  pub community_description: String,
  pub platform_url: String,
}
#[derive(Default, Clone, Archive, Serialize, Deserialize, SarSerialize, SarDeserialize)]
pub struct PlatformPatch {
  pub tenant_id: EntityId,
  pub community_name: Option<String>,
  pub community_description: Option<String>,
  pub platform_url: Option<String>
}

impl Patch<Platform> for PlatformPatch {
  fn apply_to(self, target: &mut Platform) -> () {
    if let Some(community_name) = self.community_name {
      target.community_name = community_name;
    }
    if let Some(community_description) = self.community_description {
      target.community_description = community_description;
    }
    if let Some(platform_url) = self.platform_url {
      target.platform_url = platform_url;
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
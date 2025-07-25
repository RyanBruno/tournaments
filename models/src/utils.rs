#[cfg(not(target_arch = "wasm32"))]
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
#[cfg(not(target_arch = "wasm32"))]
use argon2::{PasswordHash, PasswordVerifier};

pub type EntityId = String;

pub trait Patch<T> {
  fn apply_to(self, target: &mut T);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn hash_password(password: &str) -> String {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
  hash
}

#[cfg(not(target_arch = "wasm32"))]
pub fn verify_password(password: &str, hash: &str) -> bool {
  let parsed_hash = PasswordHash::new(&hash);
  if let Ok(parsed) = parsed_hash {
    Argon2::default()
      .verify_password(password.as_bytes(), &parsed)
      .is_ok()
  } else {
    false
  }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn hash_password_changes_input() {
        let password = "hunter2";
        let hashed = hash_password(password);
        assert_ne!(password, hashed, "hashed password should differ from input");
    }

    #[test]
    fn verify_password_checks_correctness() {
        let password = "correcthorsebatterystaple";
        let hashed = hash_password(password);

        assert!(verify_password(password, &hashed));
        assert!(!verify_password("wrong-password", &hashed));
    }
}

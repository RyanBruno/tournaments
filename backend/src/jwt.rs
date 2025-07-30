use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string())
}

pub fn generate(email: &str) -> Result<String, Box<dyn Error>> {
    let exp = SystemTime::now()
        .checked_add(Duration::from_secs(24 * 3600))
        .expect("valid timestamp")
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs() as usize;
    let claims = Claims {
        sub: email.to_owned(),
        exp,
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret().as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims, Box<dyn Error>> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

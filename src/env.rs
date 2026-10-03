use argon2::{Argon2, PasswordHasher};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::env::var;

pub struct EnvConfiguration {
    pub username: String,
    pub password_hash: String,
    pub cookie_key: Vec<u8>,
}

impl EnvConfiguration {
    pub fn get() -> Self {
        let username = var("USERNAME").expect("USERNAME environmental variable must be set");
        let password = var("PASSWORD").expect("PASSWORD environmental variable must be set");
        let cookie_key = BASE64
            .decode(var("COOKIEKEY").expect("COOKIEKEY environmental variable must be set"))
            .expect("COOKIEKEY environmental variable must be base64-standard encoded");

        if cookie_key.len() != 64 {
            panic!("COOKIEKEY must be a 64 byte(!) base64-standard encoded string")
        }

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes())
            .expect("Argon2 hashing the provided password must succed at app startup")
            .to_string();

        EnvConfiguration {
            username,
            password_hash,
            cookie_key,
        }
    }
}

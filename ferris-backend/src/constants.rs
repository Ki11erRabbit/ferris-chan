use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;

pub fn hash_password(password: &str) -> String {
    let password = password.as_bytes();

    let salt_var = std::env::var("SALT_VAR").unwrap_or(String::from("gQTkqRTEe7urCKF+vBVllg"));

    let salt = SaltString::from_b64(&salt_var).expect("salt_error");

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt).expect("hash_error");

    password_hash.to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let password = password.as_bytes();
    let argon2 = Argon2::default();

    let hash = PasswordHash::new(hash).expect("hash_error");

    match argon2.verify_password(password, &hash) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("{:?}", e);
            false
        }
    }
}
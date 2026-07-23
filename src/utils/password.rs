use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::SaltString;
use rand_core::OsRng;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(hash).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

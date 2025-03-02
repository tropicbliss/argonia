mod utils;

use argon2::{password_hash::SaltString, PasswordVerifier};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main_js() {
    utils::set_panic_hook();
}

/// Verifies a password against a hashed password string.
///
/// # Note
/// This function blocks the main thread. For better performance, execute it in a web worker instead.
///
/// # Arguments
///
/// * `hashed` - A string containing the hashed password in PHC format
/// * `password` - The plain text password to verify
///
/// # Returns
///
/// * Returns `true` if the password matches the hash, `false` if it doesn't match
///
/// # Errors
///
/// Returns an error if:
/// * The hashed password string is not in valid PHC format
/// * The hash parameters are invalid
#[wasm_bindgen]
pub fn verify(hashed: &str, password: &str) -> Result<bool, JsError> {
    let parsed_hash = PasswordHash::new(hashed)
        .map_err(|err| JsError::new(&format!("Invalid hashed password: {err}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Hashes a password using Argon2id with secure parameters.
///
/// # Note
/// This function blocks the main thread. For better performance, execute it in a web worker instead.
///
/// # Arguments
///
/// * `password` - The plain text password to hash
///
/// # Returns
///
/// * Returns the hashed password in PHC format
///
/// # Security
///
/// This function:
/// * Uses Argon2id variant
/// * Generates a cryptographically secure random salt
/// * Uses a 32-byte output length
/// * Uses system-recommended memory and time parameters
///
/// # Errors
///
/// Returns an error if:
/// * Password hashing fails due to system resource constraints
/// * The system fails to generate secure random values for the salt
#[wasm_bindgen]
pub fn hash(password: &str) -> Result<String, JsError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| JsError::new(&err.to_string()))?
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify() {
        let password = "argonia";
        let hashed = hash(password).expect("Failed to hash password");
        let is_valid = verify(&hashed, password).expect("Unable to verify password");
        assert!(is_valid);
    }
}

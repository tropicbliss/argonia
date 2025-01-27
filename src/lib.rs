mod utils;

use argon2::{password_hash::SaltString, PasswordVerifier};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, Version};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

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
    utils::set_panic_hook();
    let parsed_hash = PasswordHash::new(hashed)
        .map_err(|err| JsError::new(&format!("Invalid hashed password: {err}")))?;
    Ok(argon_with_output_len(None)
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
    utils::set_panic_hook();
    let salt = SaltString::generate(&mut OsRng);
    Ok(argon_with_output_len(Some(32))
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| JsError::new(&err.to_string()))?
        .to_string())
}

fn argon_with_output_len(output_len: Option<usize>) -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            output_len,
        )
        .unwrap(),
    )
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

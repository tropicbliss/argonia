mod utils;

use argon2::{password_hash::SaltString, PasswordVerifier};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, Version};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn verify(hashed: &str, password: &str) -> Result<bool, JsError> {
    utils::set_panic_hook();
    let parsed_hash = PasswordHash::new(hashed)
        .map_err(|err| JsError::new(&format!("Invalid hashed password: {err}")))?;
    Ok(argon_with_output_len(None)
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

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
        let password = "argonian";
        let hashed = hash(password).expect("Failed to hash password");
        let is_valid = verify(&hashed, password).expect("Unable to verify password");
        assert!(is_valid);
    }
}

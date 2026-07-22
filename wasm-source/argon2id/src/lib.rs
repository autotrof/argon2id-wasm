use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

const MEMORY_COST_KIB: u32 = 19_456;
const TIME_COST: u32 = 2;
const PARALLELISM: u32 = 1;

fn argon2id() -> Result<Argon2<'static>, JsValue> {
    let params = Params::new(MEMORY_COST_KIB, TIME_COST, PARALLELISM, None)
        .map_err(|err| JsValue::from_str(&format!("invalid argon2 params: {err}")))?;

    Ok(Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    ))
}

#[wasm_bindgen(start)]
pub fn start() {}

#[wasm_bindgen]
pub fn hash_password(password: &str) -> Result<String, JsValue> {
    let salt = SaltString::generate(&mut OsRng);

    argon2id()?
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| JsValue::from_str(&format!("hash failed: {err}")))
}

#[wasm_bindgen]
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, JsValue> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|err| JsValue::from_str(&format!("invalid password hash: {err}")))?;

    Ok(argon2id()?
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

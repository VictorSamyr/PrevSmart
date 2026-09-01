use argon2::{
    Argon2,
    password_hash::{
        Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

pub fn generate_password_hash(password_string: &str) -> Result<String, Error> {
    let password = password_string.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt)?;

    Ok(password_hash.to_string())
}

pub fn verify_password_hash(hashed: &str, password_string: &str) -> Result<bool, Error> {
    let password = password_string.as_bytes();
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed)?;

    Ok(argon2.verify_password(password, &parsed_hash).is_ok())
}

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn generate_password_hash(password_string: &str) -> String {
    let password = password_string.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error ao Gerar Hash da Senha");

    return password_hash.to_string();
}

pub fn verify_password_hash(hashed: &str, password_string: &str) -> bool {
    let password = password_string.as_bytes();
    let argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(&hashed) {
        Ok(h) => h,
        Err(error) => panic!("Erro ao Verificar Hash da Senha: {error:?}"),
    };

    match argon2.verify_password(password, &parsed_hash) {
        Ok(_) => true,
        Err(_) => false,
    }
}

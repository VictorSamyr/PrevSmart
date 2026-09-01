use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct LoginResquestPayload {
    pub user_name: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LoginResponsePayload {
    pub auth_token: String,
}

pub struct User {
    pub id_usuario: i32,
    pub tp_usuario: i32,
    pub nome_usuario: String,
    pub hash_senha_usuario: String,
}

#[derive(Serialize)]
pub struct User_res {
    pub id_usuario: i32,
    pub tp_usuario: i32,
    pub nome_usuario: String,
    pub hash_senha_usuario: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

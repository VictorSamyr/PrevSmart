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

pub(crate) struct UserRecord {
    pub(crate) id_usuario: i32,
    pub(crate) tp_usuario: i32,
    pub(crate) nome_usuario: String,
    pub(crate) hash_senha_usuario: String,
}

#[derive(Serialize)]
pub struct PublicUser {
    pub id_usuario: i32,
    pub tp_usuario: i32,
    pub nome_usuario: String,
}

impl From<UserRecord> for PublicUser {
    fn from(user: UserRecord) -> Self {
        Self {
            id_usuario: user.id_usuario,
            tp_usuario: user.tp_usuario,
            nome_usuario: user.nome_usuario,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

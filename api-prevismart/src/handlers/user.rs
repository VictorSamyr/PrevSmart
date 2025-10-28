use crate::models::{AppState, LoginResponsePayload, LoginResquestPayload, User, User_res};
use axum::{Json, extract::State};
use axum_macros::debug_handler;
use sqlx::{Pool, Postgres, Row};

pub async fn get_user(pool: &Pool<Postgres>, user_name: &str) -> User {
    let q = format!(
        "SELECT id_usuario, tp_usuario, nome_usuario, hash_senha_usuario FROM TB_USUARIO WHERE nome_usuario = '{user_name}'"
    );

    let row = sqlx::query(&q)
        .fetch_one(pool)
        .await
        .unwrap_or_else(|error| {
            panic!("Erro ao Consultar Dados do Usuario: {error:?}");
        });

    let user: User = User {
        id_usuario: row.get("id_usuario"),
        tp_usuario: row.get("tp_usuario"),
        nome_usuario: row.get("nome_usuario"),
        hash_senha_usuario: row.get("hash_senha_usuario"),
    };

    user
}

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginResquestPayload>,
) -> Json<User_res> {
    let u = get_user(&state.pool, &payload.user_name).await;

    let res = User_res {
        id_usuario: u.id_usuario,
        tp_usuario: u.tp_usuario,
        nome_usuario: u.nome_usuario,
        hash_senha_usuario: u.hash_senha_usuario,
    };

    Json(res)
}

use crate::models::{AppState, LoginResquestPayload, PublicUser, UserRecord};
use axum::{Json, extract::State, http::StatusCode};
use axum_macros::debug_handler;
use sqlx::{Pool, Postgres, Row};

const FIND_USER_BY_NAME: &str = r#"
    SELECT id_usuario, tp_usuario, nome_usuario, hash_senha_usuario
    FROM TB_USUARIO
    WHERE nome_usuario = $1
"#;

async fn get_user(
    pool: &Pool<Postgres>,
    user_name: &str,
) -> Result<Option<UserRecord>, sqlx::Error> {
    let row = sqlx::query(FIND_USER_BY_NAME)
        .bind(user_name)
        .fetch_optional(pool)
        .await?;

    row.map(|row| {
        Ok(UserRecord {
            id_usuario: row.try_get("id_usuario")?,
            tp_usuario: row.try_get("tp_usuario")?,
            nome_usuario: row.try_get("nome_usuario")?,
            hash_senha_usuario: row.try_get("hash_senha_usuario")?,
        })
    })
    .transpose()
}

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginResquestPayload>,
) -> Result<Json<PublicUser>, StatusCode> {
    if payload.user_name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = get_user(&state.pool, &payload.user_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(user.into()))
}

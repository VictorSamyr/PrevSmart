use serde::Deserialize;

#[derive(Deserialize)]
struct LoginPayload {
    pub user_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Payloads {
    pub login_payload: LoginPayload,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDTO {
    pub token: String,
    pub username: String,
    pub nickname: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegisterDTO {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct XDripCfgDTO {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sgv {
    pub sgv: String,
    pub delta: f32,
    pub direction: String,
    pub time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VerifyCodeDTO {
    pub phone: String,
    pub nonce: String,
}
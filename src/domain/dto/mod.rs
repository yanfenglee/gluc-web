use serde::{Deserialize, Serialize};
use crate::domain::entity::{User};

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
    pub app_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sgv {
    pub sgv: String,
    pub delta: f32,
    pub direction: String,
    pub time: i64,
}
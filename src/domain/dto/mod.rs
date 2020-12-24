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

impl From<&UserRegisterDTO> for User {
    fn from(dto: &UserRegisterDTO) -> Self {
        User {
            id: 0,
            username: dto.username.clone(),
            password: Some(dto.password.clone()),
            nickname: dto.nickname.clone(),
            email: dto.email.clone(),
            phone: dto.phone.clone(),
            token: None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct XDripCfgDTO {
    pub url: String,
    pub app_type: String,
}

use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};


#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub token: String,
}
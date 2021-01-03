
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

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct Cgm {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub device: Option<String>,
    pub date: Option<i64>,
    pub date_str: Option<String>,
    pub sgv: Option<i32>,
    pub delta: Option<f32>,
    pub direction: Option<String>,
    pub type1: Option<String>,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<i32>,
    pub noise: Option<i32>,
    pub sys_time: Option<String>,
    pub utc_offset: Option<i32>,
    pub slope: Option<f64>,
    pub intercept: Option<f64>,
    pub scale: Option<i32>,
    pub mbg: Option<f64>,
}
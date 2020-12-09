
use chrono::NaiveDateTime;
use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct Cgm {
    pub id: Option<i64>,
    pub device: String,
    pub date: i64,
    pub date_str: String,
    pub sgv: i32,
    pub delta: f32,
    pub direction: String,
    pub type1: String,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<i32>,
    pub noise: Option<i32>,
    pub sys_time: Option<NaiveDateTime>,
    pub utc_offset: i32,
    pub slope: f64,
    pub intercept: f64,
    pub scale: i32,
    pub mbg: i32,
}
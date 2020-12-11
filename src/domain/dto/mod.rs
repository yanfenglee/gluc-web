use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// blood glucose dto
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BgDTO {
    pub id: Option<i64>,
    pub device: String,
    pub date: i64,
    pub dateString: String,
    pub sgv: i32,
    pub delta: f32,
    pub direction: String,
    pub type1: String,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<i32>,
    pub noise: Option<i32>,
    pub sysTime: Option<NaiveDateTime>,
    pub utcOffset: i32,
    pub slope: f64,
    pub intercept: f64,
    pub scale: i32,
    pub mbg: f64,
}
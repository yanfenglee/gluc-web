use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::BgDTO;
use crate::domain::entity::Cgm;

///Cgm service
pub struct CgmService {}

impl CgmService {
    pub async fn add(&self, arg: &BgDTO) -> Result<u64> {
        let cgm = Cgm {
            id: Some(rbatis::plugin::snowflake::async_snowflake_id().await),
            device: "".to_string(),
            date: 0,
            date_str: "".to_string(),
            sgv: 0,
            delta: 0.0,
            direction: "".to_string(),
            type1: "".to_string(),
            filtered: None,
            unfiltered: None,
            rssi: None,
            noise: None,
            sys_time: None,
            utc_offset: 0,
            slope: 0.0,
            intercept: 0.0,
            scale: 0,
            mbg: 0
        };
        Ok(RB.save("", &cgm).await?.rows_affected)
    }
}

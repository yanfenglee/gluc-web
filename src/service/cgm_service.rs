use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::BgDTO;
use crate::domain::entity::Cgm;
use actix_web::guard::Guard;

///Cgm service
pub struct CgmService {}

impl CgmService {
    pub async fn add(&self, arg: &BgDTO) -> Result<u64> {
        let cgm = Cgm {
            id: Some(rbatis::plugin::snowflake::async_snowflake_id().await),
            device: arg.device.clone(),
            date: arg.date,
            date_str: arg.dateString.clone(),
            sgv: arg.sgv,
            delta: arg.delta,
            direction: arg.direction.clone(),
            type1: arg.type1.clone(),
            filtered: arg.filtered,
            unfiltered: arg.unfiltered,
            rssi: arg.rssi,
            noise: arg.noise,
            sys_time: arg.sysTime,
            utc_offset: arg.utcOffset,
            slope: arg.slope,
            intercept: arg.intercept,
    scale: arg.scale,
            mbg: arg.mbg
        };
        Ok(RB.save("", &cgm).await?.rows_affected)
    }


    pub async fn list(&self, ts: i64, cnt: i64) -> Result<Vec<BgDTO>> {

        //let w = RB.new_wrapper().ge("date", rr).check()?;
        //let ret: Result<Vec<Cgm>> = RB.list_by_wrapper("", &w).await;

        #[py_sql(RB, "SELECT * FROM cgm WHERE `date` > #{ts} LIMIT #{cc}")]
        fn select_entries(ts: i64, cc: i64) -> Vec<Cgm> {}

        let cgms = select_entries(ts, cnt).await?;

        Ok(cgms.iter().map(|x| x.into()).collect())
    }
}

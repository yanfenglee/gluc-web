use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::BgDTO;
use crate::domain::entity::Cgm;
use actix_web::guard::Guard;
use std::ops::Deref;

///Cgm service
pub struct CgmService {}

impl CgmService {
    pub async fn add(&self, arg: &Vec<BgDTO>) -> Result<u64> {
        let entries: Vec<Cgm> = arg.iter().map(|item| item.into()).collect();

        Ok(RB.save_batch("", &entries).await?.rows_affected)
    }


    pub async fn list(&self, ts: i64, cnt: i64) -> Result<Vec<BgDTO>> {

        //let w = RB.new_wrapper().ge("date", rr).check()?;
        //let ret: Result<Vec<Cgm>> = RB.list_by_wrapper("", &w).await;

        #[py_sql(RB, "SELECT * FROM cgm WHERE `date` < #{ts} order by `date` desc LIMIT #{cc}")]
        fn select_entries(ts: i64, cc: i64) -> Vec<Cgm> {}

        let cgms = select_entries(ts, cnt).await?;

        Ok(cgms.iter().map(|x| x.into()).collect())
    }
}

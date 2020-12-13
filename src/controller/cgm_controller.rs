use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::domain::entity::{Cgm};
use crate::domain::vo::RespVO;
use crate::service::CGM_SERVICE;
use crate::domain::dto::BgDTO;


/// receive bg
#[post("/entries")]
pub async fn receiveBG(mut arg: web::Json<BgDTO>) -> impl Responder {

    let data = CGM_SERVICE.add(&arg).await;
    RespVO::from_result(&data).resp()
}


#[derive(Debug, Deserialize)]
struct Info {
    count: i64,
    rr: i64,
}

#[get("/entries")]
pub async fn list(info: web::Query<Info>) -> impl Responder {

    log::info!("query entries {:?}", info);

    let data = CGM_SERVICE.list(info.rr, info.count).await;
    RespVO::from_result(&data).resp()
}


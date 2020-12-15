use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::domain::entity::{Cgm};
use crate::domain::vo::RespVO;
use crate::service::CGM_SERVICE;
use crate::domain::dto::BgDTO;


/// receive bg
#[post("/api/v1/entries")]
pub async fn receive_bg(arg: web::Json<Vec<BgDTO>>) -> impl Responder {
    log::info!("receive bg: {:?}", arg);

    let data = CGM_SERVICE.add(&arg).await;
    RespVO::from_result(&data).resp()
}


#[derive(Debug, Deserialize)]
struct Info {
    count: i64,
    rr: i64,
}

#[get("/api/v1/entries")]
pub async fn list(info: web::Query<Info>) -> impl Responder {

    log::info!("query entries {:?}", info);

    let data = CGM_SERVICE.list(info.rr, info.count).await;
    RespVO::from_result(&data).resp()
}


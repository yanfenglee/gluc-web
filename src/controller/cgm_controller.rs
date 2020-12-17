use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::domain::entity::{Cgm};
use crate::domain::vo::RespVO;
use crate::service::CGM_SERVICE;
use crate::domain::dto::BgDTO;
use rbatis::core::Error;


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

#[get("/api/v1/entries.json")]
pub async fn get_bg(info: web::Query<Info>) -> impl Responder {

    log::info!("query entries {:?}", info);

    let data = CGM_SERVICE.list(info.rr, info.count).await;
    RespVO::from_result(&data).resp()
}

#[post("/api/v1/devicestatus")]
pub async fn device_status(arg: web::Json<Vec<BgDTO>>) -> impl Responder {
    log::info!("receive bg: {:?}", arg);

    let data = CGM_SERVICE.add(&arg).await;
    RespVO::from_result(&data).resp()
}

#[get("/api/v1/devicestatus.json")]
pub async fn list_device_status(info: web::Query<Info>) -> impl Responder {

    log::info!("query entries {:?}", info);

    let data = CGM_SERVICE.list(info.rr, info.count).await;
    RespVO::from_result(&data).resp()
}

#[get("/api/v1/treatments")]
pub async fn get_treatments() -> impl Responder {

    let data: Result<Vec<i32>, Error> = Ok(vec![]);
    RespVO::from_result(&data).resp()
}


use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;

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

#[get("/entries/{rr}")]
pub async fn list(rr: web::Path<i64>) -> impl Responder {

    log::info!("query entries {}", rr);

    let data = CGM_SERVICE.list(rr.into_inner()).await;
    RespVO::from_result(&data).resp()
}


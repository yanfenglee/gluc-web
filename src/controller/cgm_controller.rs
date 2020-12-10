use actix_web::{Responder, web};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;

use crate::domain::entity::{Cgm};
use crate::domain::vo::RespVO;
use crate::service::CGM_SERVICE;
use crate::domain::dto::BgDTO;


/// receive bg
pub async fn receiveBG(mut arg: web::Json<BgDTO>) -> impl Responder {

    let data = CGM_SERVICE.add(&arg).await;
    RespVO::from_result(&data).resp()
}


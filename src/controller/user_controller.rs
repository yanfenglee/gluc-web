use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::domain::entity::{Cgm};
use crate::domain::vo::RespVO;
use crate::service::CGM_SERVICE;
use crate::domain::dto::BgDTO;
use rbatis::core::Error;
use crate::middleware::auth_user::AuthUser;
use crate::middleware::auth;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
        .service(register)
        .service(login)
        .wrap(auth::UserAuth)
        .service(xdrip_config)
    );
}


#[post("/register")]
pub async fn register(arg: web::Json<RegisterDTO>) -> impl Responder {
    log::info!("user register: {:?}", arg);

    let data = CGM_SERVICE.add(&arg, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}

#[post("/login")]
pub async fn login(arg: web::Json<LoginDTO>) -> impl Responder {
    log::info!("user login: {:?}", arg);

    let data = CGM_SERVICE.add(&arg, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}

#[get("/xdrip/config")]
pub async fn xdrip_config(user: Option<AuthUser>) -> impl Responder {
    log::info!("query entries {:?}, {:?}", user, info);

    let data = CGM_SERVICE.list(info.rr, info.count, user.unwrap().user_id).await;
    RespVO::from_result(&data).resp()
}

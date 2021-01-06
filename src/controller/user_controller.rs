use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::base::resp::resp;
use crate::service::USER_SERVICE;
use crate::base::resp::Result;
use crate::middleware::auth_user::AuthUser;
use crate::middleware::auth;
use crate::domain::dto::{UserRegisterDTO, UserLoginDTO};

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user")
        .service(register)
        .service(login)
    )
        .service(web::scope("/")
            .wrap(auth::UserAuth)
            .service(xdrip_config)
        );
}


#[post("/register")]
pub async fn register(arg: web::Json<UserRegisterDTO>) -> impl Responder {
    log::info!("user register: {:?}", arg);

    let data = USER_SERVICE.register(&arg).await;
    resp(&data)
}

#[post("/login")]
pub async fn login(arg: web::Json<UserLoginDTO>) -> impl Responder {
    log::info!("user login: {:?}", arg);

    let data = USER_SERVICE.login(&arg).await;
    resp(&data)
}

#[get("/xdrip/config")]
pub async fn xdrip_config(user: Option<AuthUser>) -> impl Responder {
    log::info!("query entries {:?}", user);

    let data = USER_SERVICE.get_xdrip_cfg(user.unwrap().user_id).await;
    resp(&data)
}

use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::base::resp::{resp, resp_html};
use crate::service::USER_SERVICE;
use crate::base::resp::Result;
use crate::middleware::auth_user::AuthUser;
use crate::middleware::auth;
use crate::domain::dto::{UserRegisterDTO, UserLoginDTO};
use crate::view::index::Index;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/cgm").wrap(auth::UserAuth)
        .service(get_current)
    );
}


#[get("/current")]
pub async fn get_current(user: Option<AuthUser>) -> impl Responder {
    log::info!("query entries {:?}", user);

    let val = "4.5";
    let delta = "0.3";
    let direction = "flat";

    let index = Index {
        val,
        delta,
        direction,
    };

    resp_html(index)
}

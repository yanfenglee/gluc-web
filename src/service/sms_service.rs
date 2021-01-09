use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::{UserDTO, UserLoginDTO, UserRegisterDTO, XDripCfgDTO, VerifyCodeDTO};
use actix_web::guard::Guard;
use std::ops::Deref;
use crate::base::resp::RespErr::{SimpleError, CodeError};
use crate::base::resp::{Result, Resp};
use crate::domain::entity::User;
use crate::util::hash;
use crate::util::local_cache;

///User service
pub struct SmsService {}

const SEND_INTERVAL: i64 = 60000;

impl SmsService {
    pub async fn send_code(&self, arg: &VerifyCodeDTO) -> Resp<i64> {
        /// send sms code

        log::info!("register info: {:?}", arg);

        let res = Resp {
            code: "0".to_string(),
            msg: None,
            data: Some(SEND_INTERVAL)
        };

        res
    }

}

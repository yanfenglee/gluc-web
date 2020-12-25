use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::{UserDTO, UserLoginDTO, UserRegisterDTO, XDripCfgDTO};
use actix_web::guard::Guard;
use std::ops::Deref;
use crate::domain::vo::RespErr::SimpleError;
use crate::domain::vo::Result;
use crate::domain::entity::User;
use crate::util::hash;

///Cgm service
pub struct UserService {}

impl UserService {
    pub async fn register(&self, arg: &UserRegisterDTO) -> Result<String> {

        //Ok(RB.save_batch("", &entries).await?.rows_affected)
        let user = User {
            id: rbatis::plugin::snowflake::block_snowflake_id(),
            username: arg.username.clone(),
            password: Some(hash::sha1(&arg.password)),
            nickname: arg.nickname.clone(),
            email: arg.email.clone(),
            phone: arg.phone.clone(),
            token: Some(hash::sha1(&format!("{}_{}", arg.username, arg.password))),
        };

        //Ok(RB.save_batch("", &entries).await?.rows_affected)
        Err(SimpleError("not implement".to_string()))
    }

    pub async fn login(&self, arg: &UserLoginDTO) -> Result<UserDTO> {
        Err(SimpleError("not implement".to_string()))

        //Ok(RB.save_batch("", &entries).await?.rows_affected)
    }


    pub async fn get_xdrip_cfg(&self, user_id: i64) -> Result<XDripCfgDTO> {

        Err(SimpleError("not implement".to_string()))
    }
}

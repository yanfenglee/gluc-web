use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::{UserDTO, UserLoginDTO, UserRegisterDTO, XDripCfgDTO};
use actix_web::guard::Guard;
use std::ops::Deref;
use crate::domain::vo::RespErr::{SimpleError, CodeError};
use crate::domain::vo::Result;
use crate::domain::entity::User;
use crate::util::hash;

///User service
pub struct UserService {}

impl UserService {
    pub async fn register(&self, arg: &UserRegisterDTO) -> Result<u64> {
        let user = User {
            id: rbatis::plugin::snowflake::block_snowflake_id(),
            username: arg.username.clone(),
            password: Some(hash::sha1(&arg.password)),
            nickname: arg.nickname.clone(),
            email: arg.email.clone(),
            phone: arg.phone.clone(),
            token: hash::sha1(&format!("{}_{}", arg.username, arg.password)),
        };

        log::info!("register info: {:?}", user);

        Ok(RB.save("", &user).await?.rows_affected)
    }

    pub async fn login(&self, arg: &UserLoginDTO) -> Result<UserDTO> {
        let hash_pass = hash::sha1(&arg.password);

        let wrapper = RB.new_wrapper()
            .eq("username", arg.username.clone())
            .and()
            .eq("password", hash_pass)
            .check()?;

        if let Ok(user) = RB.fetch_by_wrapper::<User>("", &wrapper).await {
            Ok(UserDTO {
                token: user.token,
                username: user.username,
                nickname: user.nickname,
            })
        } else {
            Err(CodeError("2".into(), "用户名或密码错误".into()))
        }
    }


    pub async fn get_xdrip_cfg(&self, user_id: i64) -> Result<XDripCfgDTO> {
        Err(SimpleError("not implement".to_string()))
    }
}

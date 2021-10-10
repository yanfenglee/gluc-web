
use rbatis::crud::CRUD;

use crate::dao::RB;
use crate::domain::dto::{UserDTO, UserLoginDTO, UserRegisterDTO, XDripCfgDTO};

use crate::base::resp::RespErr::{CodeError};
use crate::base::resp::Result;
use crate::domain::entity::User;
use crate::util::hash;

///User service
pub struct UserService {}

impl UserService {
    pub async fn register(&self, arg: &UserRegisterDTO) -> Result<u64> {
        let wrapper = RB.new_wrapper()
            .eq("username", &arg.username);

        if let Ok(user) = RB.fetch_by_wrapper::<User>(wrapper).await {
            return Err(CodeError("3".into(), "用户名已经存在".into()));
        }

        let user = User {
            id: rbatis::plugin::snowflake::new_snowflake_id(),
            username: arg.username.clone(),
            password: Some(arg.password.clone()),
            nickname: arg.nickname.clone(),
            email: arg.email.clone(),
            phone: arg.phone.clone(),
            token: hash::sha1(&format!("{}_{}", arg.username, arg.password)),
        };

        log::info!("register info: {:?}", user);

        Ok(RB.save(&user, &[]).await?.rows_affected)
    }

    pub async fn login(&self, arg: &UserLoginDTO) -> Result<UserDTO> {
        //let hash_pass = hash::sha1(&arg.password);

        let wrapper = RB.new_wrapper()
            .eq("username", &arg.username)
            .and()
            .eq("password", &arg.password);

        if let Ok(user) = RB.fetch_by_wrapper::<User>(wrapper).await {
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
        // Err(SimpleError("not implement".to_string()))

        let wrapper = RB.new_wrapper()
            .eq("id", &user_id);

        let user = RB.fetch_by_wrapper::<User>(wrapper).await?;

        Ok(XDripCfgDTO {
            url: format!("https://{}_{}@gluc.cn/api/v1/", user.username, user.password.unwrap())
        })
    }
}

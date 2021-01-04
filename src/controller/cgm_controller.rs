use actix_web::{Responder, web, get, post};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use serde::Deserialize;
use chrono;

use crate::base::resp::{resp, resp_html};
use crate::service::USER_SERVICE;
use crate::base::resp::Result;
use crate::middleware::auth_user::AuthUser;
use crate::middleware::auth;
use crate::domain::dto::{UserRegisterDTO, UserLoginDTO, Sgv};
use crate::view::index::Index;
use crate::domain::entity::Cgm;
use crate::dao::RB;
use rbatis::crud::CRUD;
use crate::util::arrow::ARROW;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/cgm").wrap(auth::UserAuth)
        .service(get_current)
    );
}


#[get("/current")]
pub async fn get_current(user: Option<AuthUser>) -> impl Responder {
    log::info!("query entries {:?}", user);

    let user_id = user.unwrap().user_id;

    return if let Ok(Some(rd)) = select_one(user_id).await {
        let t = chrono::NaiveDateTime::from_timestamp(rd.time/1000, 0)
            .format("%Y-%m-%d %H:%M:%S").to_string();

        let index = Index {
            bg: rd.sgv,
            delta: rd.delta,
            direction: ARROW.get(&rd.direction.as_str()).unwrap_or(&"").to_string(),
            time: t,
        };

        resp_html(index)
    } else {
        resp_html(Index::default())
    }

}


#[py_sql(RB, "SELECT ROUND(sgv/18.0,1) sgv, ROUND(delta/18.0,1) delta, direction, date `time` \
    FROM cgm WHERE user_id = #{user_id} and sgv is not null order by `date` desc LIMIT 1")]
async fn select_one(user_id: i64) -> Option<Sgv> {}

#[cfg(test)]
mod test {
    use crate::controller::cgm_controller::select_one;
    use crate::config::CONFIG;
    use crate::dao::RB;

    #[async_std::test]
    async fn test_sql() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None,true).unwrap();
        RB.link(&CONFIG.mysql_url).await.unwrap();

        let res = select_one(185675857577250816).await;
        println!("res : {:?}", res)
    }
}


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
use crate::domain::entity::Cgm;
use crate::dao::RB;
use rbatis::crud::CRUD;

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

    //
    // #[py_sql(RB, "SELECT * FROM cgm WHERE user_id = #{user_id} order by `date` desc LIMIT 1")]
    // fn select_entries(user_id: i64) -> Option<Cgm> {}
    //
    // let cgm = select_entries(user.unwrap().user_id).await.unwrap().unwrap();

    let wrapper = RB.new_wrapper()
        .eq("user_id", user_id)
        .order_by(false, &[&"date"])
        .check().unwrap();

    if let Ok(cgm) = RB.fetch_by_wrapper::<Cgm>("", &wrapper).await {

        let index = Index {
            val: cgm.sgv.unwrap().to_string(),
            delta: cgm.delta.unwrap().to_string(),
            direction: cgm.direction.unwrap(),
        };

        return resp_html(index);
    } else {

        return resp_html(Index {
            val: "".to_string(),
            delta: "".to_string(),
            direction: "".to_string()
        });
    }

}

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

use config::CONFIG;
use controller::{cgm_controller};
use dao::RB;
use actix_web::ResponseError;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello cgm")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    //ORM
    RB.link(&CONFIG.mysql_url).await.unwrap();
    //路由
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}

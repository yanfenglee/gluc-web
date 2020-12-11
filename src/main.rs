use actix_web::{App, HttpResponse, HttpServer, Responder, web, ResponseError};

use gluc::config::{log, CONFIG};
use gluc::controller::{cgm_controller};
use gluc::dao::RB;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello cgm")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init_log();

    RB.link(&CONFIG.mysql_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(cgm_controller::receiveBG)
            .service(cgm_controller::list)
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}

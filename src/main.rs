use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web, ResponseError, HttpMessage, Either};

use gluc_web::config::{log_config, CONFIG};
use gluc_web::controller::{user_controller};
use gluc_web::dao::RB;
use actix_http::http::Method;
use actix_web::dev::{Service, ServiceResponse};
use futures::FutureExt;
use actix_http::{Error, Response};
use gluc_web::middleware::auth;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello cgm")
}

async fn default_proc(req: HttpRequest, body: web::Bytes) -> impl Responder {
    log::info!("\n----------------------------------------\n");
    log::info!("req: {:?}", req);

    if let Ok(result) = std::str::from_utf8(&body) {
        log::info!("body: {:?}", result);
    }

    HttpResponse::Ok().body("Hello cgm")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_config::init_log();

    RB.link(&CONFIG.mysql_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .configure(user_controller::config)
            .default_service(web::route().to(default_proc)
            )
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}

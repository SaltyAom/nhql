#[macro_use]
extern crate lazy_static;

mod modules;
mod models;

use actix_web::{ HttpServer, App, middleware::Compress, http };
use actix_cors::Cors;

use modules::{
    landing::controller::landing_module,
    proxy::controller::proxy_module
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .max_age(86400);

        App::new()
            .wrap(cors)
            .wrap(Compress::default())
            .configure(landing_module)
            .configure(proxy_module)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
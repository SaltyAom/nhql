mod modules;
mod models;

use modules::{
    landing::controller::landing_module,
    proxy::controller::proxy_module
};

use actix_web::{ HttpServer, App };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(landing_module)
            .configure(proxy_module)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
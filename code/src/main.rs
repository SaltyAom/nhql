extern crate juniper;

mod modules;
mod models;
mod services;

use actix_web::{ HttpServer, App, middleware::Compress, web::route };

use modules::{
    status::controller::{ status, fallback },
    proxy::controller::proxy,
    graphql::controller::graphql
};

use services::schema::service::create_schema;

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .data(schema.clone())
            .configure(status)
            .configure(proxy)
            .configure(graphql)
            .default_service(
                route().to(fallback)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
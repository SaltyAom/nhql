extern crate juniper;

mod modules;
mod models;
mod services;

use actix_web::{ HttpServer, App, middleware::Compress, web::{ route, Data } };

use modules::{
    status::controller::{ status, fallback },
    proxy::controller::proxy,
    graphql::controller::graphql
};
use actix_cors::Cors;

use services::schema::service::create_schema;

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Compress::default())
            .app_data(Data::new(schema.clone()))
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
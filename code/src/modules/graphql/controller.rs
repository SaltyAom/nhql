use std::sync::Arc;

use actix_web::{Error, HttpResponse, web, get, post, Result, web::ServiceConfig};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::services::schema::service::Schema;

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .append_header(("Cache-Control", "max-age=259200"))
        .body(
            graphiql_source("/graphql", None)
        )
}

#[post("/graphql")]
pub async fn graphql_handler(
    data: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let res = request.execute(&data, &()).await;
    
    Ok(HttpResponse::Ok()
        .append_header(("Cache-Control", "max-age=259200"))
        .json(res)
    )
}

pub fn graphql(config: &mut ServiceConfig) {
    config
        .service(graphiql)
        .service(graphql_handler);
}
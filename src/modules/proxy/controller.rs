use actix_web::{ get, HttpResponse, web::{ Path, ServiceConfig }};

use super::service::{compose_empty_nh_api_data, search_hentai, get_hentai, is_nhentai};

#[get("/h/{id}")]
async fn proxy(Path(id): Path<String>) -> HttpResponse {
    if !is_nhentai(&id) {
        return HttpResponse::Ok()
            .json(compose_empty_nh_api_data(&id))
    }

    HttpResponse::Ok()
        .json(
            get_hentai(&id).await
        )
}

#[get("/search/{search}")]
async fn proxy_search(Path(search): Path<String>) -> HttpResponse {
    let search_result = search_hentai(&search, 1).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(
            search_result
        )
}

pub fn proxy_module(config: &mut ServiceConfig) {
    config
        .service(proxy)
        .service(proxy_search);
}

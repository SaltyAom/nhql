use actix_web::{ get, HttpResponse, web::{ Path, ServiceConfig }};

use super::service::{compose_empty_nh_api_data, search_hentai, get_hentai, is_nhentai};

#[get("/h/{id}")]
async fn proxy(path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    if !is_nhentai(id) {
        return HttpResponse::Ok()
            // .header("Cache-Control", "max-age=259200")
            .json(compose_empty_nh_api_data(id))
    }

    HttpResponse::Ok()
        .json(
            get_hentai(id).await
        )
}

#[get("/search/{search}")]
async fn proxy_search(path: Path<String>) -> HttpResponse {
    let search = path.into_inner();

    let search_result = search_hentai(search, 1).await;

    HttpResponse::Ok()
        .content_type("application/json")
        // .header("Cache-Control", "max-age=259200")
        .json(
            search_result
        )
}

#[get("/search/{search}/{page}")]
async fn proxy_search_with_page(path: Path<(String, i32)>) -> HttpResponse {
    let (search, page) = path.into_inner();

    let search_result = search_hentai(search, page).await;

    HttpResponse::Ok()
        .content_type("application/json")
        // .header("Cache-Control", "max-age=259200")
        .json(
            search_result
        )
}

pub fn proxy_module(config: &mut ServiceConfig) {
    config
        .service(proxy)
        .service(proxy_search)
        .service(proxy_search_with_page);
}

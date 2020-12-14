use actix_web::{ get, HttpResponse, web::{ Path, ServiceConfig }};

use super::service::{compose_empty_nh_api_data, search_hentai, get_hentai, map_nh_api, is_nhentai};

#[get("/{id}")]
async fn proxy(Path(id): Path<String>) -> HttpResponse {
    if !is_nhentai(&id) {
        return HttpResponse::Ok()
            .json(compose_empty_nh_api_data(&id))
    }

    let nhentai = get_hentai(&id).await;

    if nhentai.is_none() {
        return HttpResponse::Ok()
            .json(compose_empty_nh_api_data(&id))
    }

    let nh_api = map_nh_api(nhentai.unwrap());

    HttpResponse::Ok()
        .content_type("application/json")
        .json(
            nh_api
        )
}

#[get("/search/{search}")]
async fn proxy_search(Path(search): Path<String>) -> HttpResponse {
    let search_result = search_hentai(&search, 1).await;

    if search_result.is_none() {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body("[]")
    }

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

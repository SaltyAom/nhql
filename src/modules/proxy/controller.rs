use actix_web::{ get, HttpResponse, web::{ Path, ServiceConfig }};

use super::service::{compose_empty_nh_api_data, get_hentai, map_nh_api, is_nhentai};

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

pub fn proxy_module(config: &mut ServiceConfig) {
    config
        .service(proxy);
}

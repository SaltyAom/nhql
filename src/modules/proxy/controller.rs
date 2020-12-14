use actix_web::{ get, HttpResponse, web::{ Path, ServiceConfig }};

use super::service::get_hentai;

#[get("/h/{id}")]
async fn proxy(Path(id): Path<String>) -> HttpResponse {
    let nhentai = get_hentai(&id).await;

    if nhentai.is_none() {
        return HttpResponse::InternalServerError()
            .body("Internal Server Error")
    }
 
    HttpResponse::Ok()
        .content_type("application/json")
        .json(
            nhentai.unwrap()
        )
}

pub fn proxy_module(config: &mut ServiceConfig) {
    config
        .service(proxy);
}
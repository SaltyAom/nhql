use actix_web::{ get, Responder, web::ServiceConfig };

#[get("/")]
async fn landing() -> impl Responder {
    "Working"
}

pub fn landing_module(config: &mut ServiceConfig) {
    config
        .service(landing);
}

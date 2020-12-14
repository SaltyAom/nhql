use actix_web::{ get, HttpResponse, web::{ Path, ServiceConfig }, Error, client::Client};

#[get("/h/{id}")]
async fn proxy(Path(id): Path<String>) -> HttpResponse {
    let client = Client::default();

    let proxy_server = format!("{}/{}", "https://nhentai.net/api/gallery", id);

    // Create request builder and send request
    let mut response = client.get(proxy_server)
       .send()
       .await
       .map_err(Error::from)
       .unwrap();

    let response_stream = response.body().await.unwrap().to_vec();
    let body = String::from_utf8(response_stream);
 
    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            body.unwrap()
        )
}

pub fn proxy_module(config: &mut ServiceConfig) {
    config
        .service(proxy);
}
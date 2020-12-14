use actix_web::client::Client;

use crate::models::nhentai::model::NHentai;

pub async fn get_hentai(id: &str) -> Option<NHentai> {
    let client = Client::default();

    let proxy_server = format!("{}/{}", "https://nhentai.net/api/gallery", id);

    // Create request builder and send request
    let response = client.get(proxy_server)
       .send()
       .await;

    if response.is_err() {
        return None
    }

    let mut response_data = response.unwrap();
    let response_stream = response_data.body().await.unwrap().to_vec();
    let body = String::from_utf8(response_stream).unwrap();

    Some(map_nhentai(&body))
}

pub fn map_nhentai(raw: &str) -> NHentai {
    let nhentai: NHentai = serde_json::from_str(raw).unwrap();

    nhentai
}
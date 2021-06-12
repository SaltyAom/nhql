use chrono::NaiveDateTime;
use cached::proc_macro::cached;

use crate::models::{
    nhapi::model::{
        NHApi, 
        NHApiArtist, 
        NHApiImages, 
        NHApiInfo, 
        NHApiInfoUpload, 
        NHApiMetadata, 
        NHApiPage, 
        NHApiPageInfo, 
        NHApiPages, 
        NHApiSearch, 
        NHApiTag, 
        NHApiTags, 
        NHApiTitle, 
        NHResponse, 
        NHSearchResponse
    }, 
    nhentai::model::{
        NHentai, 
        DynamicNHentai, 
        NHentaiGroup, 
        DynamicNHentaiGroup, 
        NHentaiImages, 
        NHentaiTags, 
        NHentaiTitle
    }
};

const NHENTAI_NOT_FOUND: &'static str = "{\"error\": \"does not exist\"}";

#[cached]
pub async fn get_hentai(id: i32) -> NHResponse {
    let proxy_server = format!("{}/{}", "https://nhentai.net/api/gallery", id);

    // Create request builder and send request
    let response = reqwest::get(&proxy_server)
       .await;

    if response.is_err() {
        return compose_empty_nh_api_data(id)
    }

    let response_data = response.unwrap();

    if response_data.status() == 404 {
        return compose_empty_nh_api_data(id)
    }

    let body = response_data.text().await.unwrap();

    if body == NHENTAI_NOT_FOUND {
        return compose_empty_nh_api_data(id)
    }

    NHResponse {
        success: true,
        info: String::from(""),
        data: map_nh_api(map_nhentai(parse_nhentai_from_string(&body)))
    }
}

#[cached]
pub async fn search_hentai(search_key: String, page: i32) -> NHSearchResponse {
    let proxy_server = format!("https://nhentai.net/api/galleries/search?query={}&page={}", search_key, page);

    // Create request builder and send request
    let response = reqwest::get(&proxy_server)
       .await;

    if response.is_err() {
        return NHSearchResponse {
            success: false,
            info: "Not found".to_owned(),
            data: vec![]
        }
    }

    let response_data = response.unwrap();
    let body = response_data.text().await.unwrap();

    let nhentai_search_response = map_nhentai_group(&body).result;
    let nhapi_search: NHApiSearch = nhentai_search_response.into_iter().map(|hentai| map_nh_api(hentai)).collect();

    NHSearchResponse {
        success: true,
        info: String::from(""),
        data: nhapi_search
    }
}

pub fn parse_nhentai_from_string(raw: &str) -> DynamicNHentai {
    let nhentai: DynamicNHentai = serde_json::from_str(raw).unwrap();

    nhentai
}

pub fn map_nhentai(nhentai: DynamicNHentai) -> NHentai {
    let stringified_id = serde_json::to_string(&nhentai.id).unwrap();
    let id: u32 = stringified_id.parse().unwrap();

    NHentai {
        id,
        media_id: nhentai.media_id,
        title: nhentai.title,
        images: nhentai.images,
        scanlator: nhentai.scanlator,
        upload_date: nhentai.upload_date,
        tags: nhentai.tags,
        num_pages: nhentai.num_pages,
        num_favorites: nhentai.num_favorites
    }
}

pub fn map_nhentai_group(raw: &str) -> NHentaiGroup {
    let dynamic_nhentai_group: DynamicNHentaiGroup = serde_json::from_str(raw).map_err(|e| {
        println!("Parse Group Error: {:?}", e);
    }).unwrap_or_else(|_|
        DynamicNHentaiGroup {
            result: vec![],
            num_pages: 0,
            per_page: 0
        }
    );

    let nhentais: Vec<NHentai> =  dynamic_nhentai_group.result
        .into_iter()
        .map(|hentai| map_nhentai(hentai))
        .collect();

    let nhentai_group: NHentaiGroup = NHentaiGroup {
        result: nhentais,
        num_pages: dynamic_nhentai_group.num_pages,
        per_page: dynamic_nhentai_group.per_page
    };

    nhentai_group
}

pub fn map_nh_api(nhentai: NHentai) -> NHApi {
    NHApi {
        id: nhentai.id as i32,
        title: map_title(nhentai.title),
        images: map_images(&nhentai.media_id, &nhentai.images),
        info: NHApiInfo {
            amount: nhentai.images.pages.len() as i32,
            favorite: nhentai.num_favorites as i32,
            upload: NHApiInfoUpload {
                original: nhentai.upload_date as i32,
                parsed: NaiveDateTime::from_timestamp(nhentai.upload_date as i64, 0).to_string()
            }
        },
        metadata: map_metadata(&nhentai.tags)
    }
}

pub fn map_title(title: NHentaiTitle) -> NHApiTitle {
    NHApiTitle {
        display: title.pretty,
        english: title.english,
        japanese: title.japanese
    }
}

pub fn map_images(media_id: &str, images: &NHentaiImages) -> NHApiImages {
    let pages: NHApiPages = images.pages.iter().enumerate().map(|(index, page)| {
        let extension = map_extension(&page.t);

        NHApiPage {
            link: compose_page_link(media_id, (index + 1) as i32, &extension),
            info: NHApiPageInfo {
                r#type: extension.to_string(),
                width: page.w as i32,
                height: page.h as i32
            }
        }
    }).collect();

    let extension = map_extension(&images.cover.t);

    NHApiImages {
        pages,
        cover: NHApiPage {
            link: compose_cover_link(media_id, extension),
            info: NHApiPageInfo {
                r#type: extension.to_string(),
                width: images.cover.w as i32,
                height: images.cover.h as i32
            }
        }
    }
}

pub fn map_extension(extension_type: &str) -> &str {
    match extension_type {
        "j" => "jpg",
        "p" => "png",
        "g" => "gif",
        _ => "jpg"
    }
}

pub fn compose_page_link(media_id: &str, page: i32, extension: &str) -> String {
    format!("https://i.nhentai.net/galleries/{}/{}.{}", media_id, page, extension)
}

pub fn compose_cover_link(media_id: &str, extension: &str) -> String {
    format!("https://t.nhentai.net/galleries/{}/cover.{}", media_id, extension)
}

pub fn map_metadata(tags: &NHentaiTags) -> NHApiMetadata {
    let mut artist = NHApiArtist {
        name: String::from(""),
        count: 0,
        url: compose_tag_url("")
    };
    let mut language: String = "translated".to_owned();

    let mut nh_api_tags: NHApiTags = vec![];

    for tag in tags.into_iter() {
        match &tag.r#type[..] {
            "artist" => {
                artist = NHApiArtist {
                    name: tag.name.to_owned(),
                    count: tag.count as i32,
                    url: compose_tag_url(&tag.url)
                }
            },
            "language" => {
                if tag.name != "translated" {
                    language = tag.name.to_owned()
                }
            },
            _ => {
                nh_api_tags.push(NHApiTag {
                    name: tag.name.to_owned(),
                    count: tag.count  as i32,
                    url: compose_tag_url(&tag.url)
                })
            }
        };
    }

    NHApiMetadata {
        artist,
        language,
        tags: nh_api_tags
    }
}

pub fn compose_tag_url(tag: &str) -> String {
    format!("https://nhentai.net{}", tag)
}

pub fn compose_empty_nh_api_data(id: i32) -> NHResponse {
    NHResponse {
        success: false,
        info: "Not found".to_owned(),
        data: NHApi {
            id,
            title: NHApiTitle {
                display: String::from(""),
                english: String::from(""),
                japanese: String::from("")
            },
            images: NHApiImages {
                pages: vec![],
                cover: NHApiPage {
                    link: String::from(""),
                    info: NHApiPageInfo {
                        r#type: String::from(""),
                        width: 0,
                        height: 0
                    }
                }
            },
            info: NHApiInfo {
                amount: 0,
                favorite: 0,
                upload: NHApiInfoUpload {
                    original: 0,
                    parsed: String::from("")
                }
            },
            metadata: NHApiMetadata {
                artist: NHApiArtist {
                    name: String::from(""),
                    count: 0,
                    url: compose_tag_url("")
                },
                tags: vec![],
                language: String::from("")
            }
        }
    }
}

pub fn is_nhentai(input: i32) -> bool {
    input < 1_000_000
}
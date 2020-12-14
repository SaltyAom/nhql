use crate::{models::{nhapi::model::{NHApi, NHApiArtist, NHApiImages, NHApiInfo, NHApiInfoUpload, NHApiMetadata, NHApiPage, NHApiPageInfo, NHApiPages, NHApiSearch, NHApiTag, NHApiTags, NHApiTitle, NH_API_PAGE_TYPES_MAP}, nhentai::model::{NHentai, NHentaiGroup, NHentaiImages, NHentaiTags, NHentaiTitle}}};

use chrono::NaiveDateTime;

use std::str::FromStr;

const NHENTAI_NOT_FOUND: &'static str = "{\"error\": \"does not exist\"}";

pub async fn get_hentai(id: &str) -> NHApi {
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

    map_nh_api(map_nhentai(&body))
}

pub async fn search_hentai(search_key: &str, page: u16) -> NHApiSearch {
    let proxy_server = format!("https://nhentai.net/api/galleries/search?query=${}&page=${}", search_key, page);

    // Create request builder and send request
    let response = reqwest::get(&proxy_server)
       .await;

    if response.is_err() {
        return vec![]
    }

    let response_data = response.unwrap();
    let body = response_data.text().await.unwrap();

    let nhentai_search_response = map_nhentai_group(&body).result;
    let nhapi_search: NHApiSearch = nhentai_search_response.into_iter().map(|hentai| map_nh_api(hentai)).collect();

    nhapi_search
}

pub fn map_nhentai(raw: &str) -> NHentai {
    let nhentai: NHentai = serde_json::from_str(raw).unwrap();

    nhentai
}

pub fn map_nhentai_group(raw: &str) -> NHentaiGroup {
    let nhentai_group: NHentaiGroup = serde_json::from_str(raw).unwrap_or(NHentaiGroup {
        result: vec![],
        num_pages: 0,
        per_page: 0
    });

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
            link: compose_page_link(media_id, &(index + 1).to_string(), &extension),
            info: NHApiPageInfo {
                r#type: extension.to_string(),
                width: page.w as i32,
                height: page.h as i32
            }
        }
    }).collect();

    let extension = map_extension(&images.cover.t);

    NHApiImages {
        pages: pages,
        cover: NHApiPage {
            link: compose_page_link(media_id, "cover", extension),
            info: NHApiPageInfo {
                r#type: extension.to_string(),
                width: images.cover.w as i32,
                height: images.cover.h as i32
            }
        }
    }
}

pub fn map_extension(extension_type: &str) -> &&str {
    let extension = NH_API_PAGE_TYPES_MAP.get(extension_type).unwrap_or_else(|| &"");

    extension
}

pub fn compose_page_link(media_id: &str, page: &str, extension: &str) -> String {
    format!("https://t.nhentai.net/galleries/{}/{}.{}", media_id, page, extension)
}

pub fn map_metadata(tags: &NHentaiTags) -> NHApiMetadata {
    let mut artist = NHApiArtist {
        name: "".to_owned(),
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
        artist: artist,
        language: language,
        tags: nh_api_tags
    }
}

pub fn compose_tag_url(tag: &str) -> String {
    format!("https://nhentai.net{}", tag)
}

pub fn compose_empty_nh_api_data(id: &str) -> NHApi {
    NHApi {
        id: u32::from_str(id).unwrap_or(0) as i32,
        title: NHApiTitle {
            display: "".to_owned(),
            english: "".to_owned(),
            japanese: "".to_owned()
        },
        images: NHApiImages {
            pages: vec![],
            cover: NHApiPage {
                link: "".to_owned(),
                info: NHApiPageInfo {
                    r#type: "".to_owned(),
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
                parsed: "".to_owned()
            }
        },
        metadata: NHApiMetadata {
            artist: NHApiArtist {
                name: "".to_owned(),
                count: 0,
                url: compose_tag_url("")
            },
            tags: vec![],
            language: "".to_owned()
        },
    }
}

pub fn is_numeric(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    return true;
}

pub fn is_nhentai(input: &str) -> bool {
    is_numeric(input) && input.len() <= 6
}
use hashbrown::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct NHApi {
    pub id: u32,
    pub title: NHApiTitle,
    pub images: NHApiImages,
    pub info: NHApiInfo,
    pub metadata: NHApiMetadata
}

#[derive(Serialize)]
pub struct NHApiTitle {
    pub display: String,
    pub english: String,
    pub japanese: String
}

#[derive(Serialize)]
pub struct NHApiImages {
    pub pages: NHApiPages,
    pub cover: NHApiPage
}

#[derive(Serialize)]
pub struct NHApiPage {
    pub link: String,
    pub info: NHApiPageInfo
}

#[derive(Serialize)]
pub struct NHApiPageInfo {
    pub r#type: String,
    pub width: u16,
    pub height: u16
}

pub type NHApiPages = Vec<NHApiPage>;

#[derive(Serialize)]
pub struct NHApiInfo {
    pub amount: u16,
    pub favorite: u32,
    pub upload: NHApiInfoUpload
}

#[derive(Serialize)]
pub struct NHApiInfoUpload {
    pub original: u32,
    pub parsed: String
}

#[derive(Serialize)]
pub struct NHApiMetadata {
    pub artist: NHApiArtist,
    pub tags: NHApiTags,
    pub language: String
}

#[derive(Serialize)]
pub struct NHApiArtist {
    pub name: String,
    pub count: u32,
    pub url: String
}

#[derive(Serialize)]
pub struct NHApiTag {
    pub name: String,
    pub count: u32,
    pub url: String
}

pub type NHApiTags = Vec<NHApiTag>;
pub type NHApiSearch = Vec<NHApi>;

lazy_static! {
    pub static ref NH_API_PAGE_TYPES_MAP: HashMap<&'static str, &'static str> = {
        let mut type_map = HashMap::new();

        type_map.insert("j", "jpg");
        type_map.insert("p", "png");
        type_map.insert("g", "gif");

        type_map
    };
}
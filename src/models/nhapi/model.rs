use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct NHApi {
    id: u32,
    title: NHApiTitle,
    images: NHApiImages,
    info: NHApiInfo,
    metadata: NHApiMetadata
}

#[derive(Serialize, Deserialize)]
pub struct NHApiTitle {
    display: String,
    english: String,
    japanese: String
}

#[derive(Serialize, Deserialize)]
pub struct NHApiImages {
    pages: NHApiPages,
    cover: NHApiPage
}

#[derive(Serialize, Deserialize)]
pub struct NHApiPage {
    link: String,
    info: NHApiPageInfo
}

#[derive(Serialize, Deserialize)]
pub struct NHApiPageInfo {
    r#type: String,
    width: u16,
    height: u16
}

pub type NHApiPages = Vec<NHApiPage>;

#[derive(Serialize, Deserialize)]
pub struct NHApiInfo {
    amount: u16,
    favorite: u32,
    upload: NHApiInfoUpload
}

#[derive(Serialize, Deserialize)]
pub struct NHApiInfoUpload {
    original: u32,
    parsed: String
}

#[derive(Serialize, Deserialize)]
pub struct NHApiMetadata {
    artist: NHApiArtist,
    tags: NHApiTags,
    language: String
}

#[derive(Serialize, Deserialize)]
pub struct NHApiArtist {
    name: String,
    count: u32,
    url: String
}

#[derive(Serialize, Deserialize)]
pub struct NHApiTag {
    name: String,
    count: u32,
    url: String
}

pub type NHApiTags = Vec<NHApiTag>;

pub const NH_API_PAGE_TYPES: [&'static str; 3] = ["jpg", "png", "gif"];
pub const NH_API_LANGUAGES: [&'static str; 4] = ["japanese", "chinese", "translated", "english"];
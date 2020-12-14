use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct NHentai {
    id: u32,
    media_id: String,
    title: NHentaiTitle,
    images: NHentaiImages,
    scanlator: String,
    upload_date: u32,
    tags: NHentaiTags,
    num_pages: u16,
    num_favorites: u32
}

#[derive(Serialize, Deserialize)]
pub struct NHentaiTitle {
    english: String,
    japanese: String,
    pretty: String
}

#[derive(Serialize, Deserialize)]
pub struct NHentaiImages {
    pages: NHentaiPages,
    cover: NHentaiPage,
    thumbnail: NHentaiPage
}

#[derive(Serialize, Deserialize)]
pub struct NHentaiPage {
    t: String,
    w: u16,
    h: u16
}

pub type NHentaiPages = Vec<NHentaiPage>;

#[derive(Serialize, Deserialize)]
pub struct NHentaiTag {
    id: u32,
    r#type: String,
    name: String,
    url: String,
    count: u32
}

pub type NHentaiTags = Vec<NHentaiTag>;

pub const NHENTAI_PAGE_TYPES: [&'static str; 3] = ["j", "p", "g"];
pub const NHENTAI_LANGUAGES: [&'static str; 4] = ["japanese", "chinese", "translated", "english"];
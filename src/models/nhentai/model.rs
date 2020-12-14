use serde::{ Deserialize };

#[derive(Deserialize)]
pub struct NHentai {
    pub id: u32,
    pub media_id: String,
    pub title: NHentaiTitle,
    pub images: NHentaiImages,
    pub scanlator: String,
    pub upload_date: u32,
    pub tags: NHentaiTags,
    pub num_pages: u16,
    pub num_favorites: u32
}

#[derive(Deserialize)]
pub struct NHentaiTitle {
    pub english: String,
    pub japanese: String,
    pub pretty: String
}

#[derive(Deserialize)]
pub struct NHentaiImages {
    pub pages: NHentaiPages,
    pub cover: NHentaiPage,
    pub thumbnail: NHentaiPage
}

#[derive(Deserialize)]
pub struct NHentaiPage {
    pub t: String,
    pub w: u16,
    pub h: u16
}

pub type NHentaiPages = Vec<NHentaiPage>;

#[derive(Deserialize)]
pub struct NHentaiTag {
    pub id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
    pub count: u32
}

pub type NHentaiTags = Vec<NHentaiTag>;

#[derive(Deserialize)]
pub struct NHentaiGroup {
    pub result: Vec<NHentai>,
    pub num_pages: u16,
    pub per_page: u8
}
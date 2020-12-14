use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct NHentaiTitle {
    pub english: String,
    pub japanese: String,
    pub pretty: String
}

#[derive(Serialize, Deserialize)]
pub struct NHentaiImages {
    pub pages: NHentaiPages,
    pub cover: NHentaiPage,
    pub thumbnail: NHentaiPage
}

#[derive(Serialize, Deserialize)]
pub struct NHentaiPage {
    pub t: String,
    pub w: u16,
    pub h: u16
}

pub type NHentaiPages = Vec<NHentaiPage>;

#[derive(Serialize, Deserialize)]
pub struct NHentaiTag {
    pub id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
    pub count: u32
}

pub type NHentaiTags = Vec<NHentaiTag>;
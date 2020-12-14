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

pub struct NHentaiTitle {
    english: String,
    japanese: String,
    pretty: String
}

pub struct NHentaiImages {
    pages: NHentaiPages,
    cover: NHentaiPage,
    thumbnail: NHentaiPage
}

pub struct NHentaiPage {
    t: String,
    w: String,
    h: String
}

pub type NHentaiPages = Vec<NHentaiPage>;

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
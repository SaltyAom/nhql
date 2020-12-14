pub struct NHApi {
    id: u32,
    title: NHApiTitle,
    images: NHApiImages,
    info: NHApiInfo,
    metadata: NHApiMetadata
}

pub struct NHApiTitle {
    display: String,
    english: String,
    japanese: String
}

pub struct NHApiImages {
    pages: NHApiPages,
    cover: NHApiPage
}

pub struct NHApiPage {
    link: String,
    info: NHApiPageInfo
}

pub struct NHApiPageInfo {
    r#type: String,
    width: u16,
    height: u16
}

pub type NHApiPages = Vec<NHApiPage>;

pub struct NHApiInfo {
    amount: u16,
    favorite: u32,
    upload: NHApiInfoUpload
}

pub struct NHApiInfoUpload {
    original: u32,
    parsed: String
}

pub struct NHApiMetadata {
    artist: NHApiArtist,
    tags: NHApiTags,
    language: String
}

pub struct NHApiArtist {
    name: String,
    count: u32,
    url: String
}

pub struct NHApiTag {
    name: String,
    count: u32,
    url: String
}

pub type NHApiTags = Vec<NHApiTag>;

pub const NH_API_PAGE_TYPES: [&'static str; 3] = ["jpg", "png", "gif"];
pub const NH_API_LANGUAGES: [&'static str; 4] = ["japanese", "chinese", "translated", "english"];
use hashbrown::HashMap;

use juniper::GraphQLObject;

use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
pub struct NHApi {
    pub id: i32,
    pub title: NHApiTitle,
    pub images: NHApiImages,
    pub info: NHApiInfo,
    pub metadata: NHApiMetadata
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiTitle {
    pub display: String,
    pub english: String,
    pub japanese: String
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiImages {
    pub pages: NHApiPages,
    pub cover: NHApiPage
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiPage {
    pub link: String,
    pub info: NHApiPageInfo
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiPageInfo {
    pub r#type: String,
    pub width: i32,
    pub height: i32
}

pub type NHApiPages = Vec<NHApiPage>;

#[derive(Serialize, GraphQLObject)]
pub struct NHApiInfo {
    pub amount: i32,
    pub favorite: i32,
    pub upload: NHApiInfoUpload
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiInfoUpload {
    pub original: i32,
    pub parsed: String
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiMetadata {
    pub artist: NHApiArtist,
    pub tags: NHApiTags,
    pub language: String
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiArtist {
    pub name: String,
    pub count: i32,
    pub url: String
}

#[derive(Serialize, GraphQLObject)]
pub struct NHApiTag {
    pub name: String,
    pub count: i32,
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
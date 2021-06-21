#![allow(non_snake_case)]
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

use crate::modules::proxy::service::get_hentai;
use crate::{
    models::nhapi::model::{NHResponse, NHSearchResponse},
    modules::proxy::service::search_hentai,
};

pub struct Query;

#[graphql_object]
impl Query {
    pub async fn getHentaiById(id: i32) -> NHResponse {
        get_hentai(id).await
    }

    pub async fn searchHentai(keyword: String, page: i32) -> NHSearchResponse {
        search_hentai(keyword, page).await
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {}, 
        EmptyMutation::new(), 
        EmptySubscription::new()
    )
}


use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

use crate::{models::nhapi::model::{NHApi, NHApiSearch}, modules::proxy::service::search_hentai};
use crate::modules::proxy::service::{ get_hentai };

pub struct Query {}

#[graphql_object]
impl Query {
    pub async fn getHentaiById(id: i32) -> NHApi {
        let code = &id.to_string();

        get_hentai(code).await
    }

    pub async fn searchHentai(keyword: String) -> NHApiSearch {
        search_hentai(&keyword, 1).await
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}
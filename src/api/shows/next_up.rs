use serde::Serialize;

use crate::{
    Client,
    error::Result,
    models::{
        PaginatedResult,
        items::{
            Item,
            query_enums::{ImageType, ItemFields},
        },
    },
};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextUpQuery {
    pub user_id: Option<String>,
    pub start_index: Option<i32>,
    pub limit: Option<i32>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub fields: Option<Vec<ItemFields>>,
    pub series_id: Option<String>,
    pub parent_id: Option<String>,
    pub enable_images: Option<bool>,
    pub image_type_limit: Option<i32>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub enable_image_types: Option<Vec<ImageType>>,
    pub enable_user_data: Option<bool>,
    pub next_up_date_cutoff: Option<String>,
    pub enable_total_record_count: Option<bool>,
    pub enable_resumable: Option<bool>,
    pub enable_rewatching: Option<bool>,
}

impl Client {
    pub async fn get_next_up(&self, query: Option<NextUpQuery>) -> Result<PaginatedResult<Item>> {
        self.get("Shows/NextUp", query).await
    }
}

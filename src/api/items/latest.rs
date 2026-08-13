use serde::Serialize;

use crate::{
    Client,
    error::Result,
    models::items::{
        Item,
        enums::ItemKind,
        query_enums::{ImageType, ItemFields},
    },
};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestItemsQuery {
    pub user_id: Option<String>,
    pub parent_id: Option<String>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub fields: Option<Vec<ItemFields>>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub include_item_types: Option<Vec<ItemKind>>,
    pub is_played: Option<bool>,
    pub enable_images: Option<bool>,
    pub image_type_limit: Option<i32>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub enable_image_types: Option<Vec<ImageType>>,
    pub enable_user_data: Option<bool>,
    pub limit: Option<i32>,
    pub group_items: Option<bool>,
}

impl Client {
    pub async fn get_latest_items(&self, query: Option<LatestItemsQuery>) -> Result<Vec<Item>> {
        self.get("Items/Latest", query).await
    }
}

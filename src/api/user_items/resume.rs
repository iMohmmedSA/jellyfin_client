use serde::Serialize;

use crate::{
    Client,
    error::Result,
    models::{
        PaginatedResult,
        items::{
            Item,
            enums::{ItemKind, MediaType},
            query_enums::{ImageType, ItemFields},
        },
    },
};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumeItemsQuery {
    pub user_id: Option<String>,
    pub start_index: Option<i32>,
    pub limit: Option<i32>,
    pub search_term: Option<String>,
    pub parent_id: Option<String>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub fields: Option<Vec<ItemFields>>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub media_types: Option<Vec<MediaType>>,
    pub enable_user_data: Option<bool>,
    pub image_type_limit: Option<i32>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub enable_image_types: Option<Vec<ImageType>>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub exclude_item_types: Option<Vec<ItemKind>>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub include_item_types: Option<Vec<ItemKind>>,
    pub enable_total_record_count: Option<bool>,
    pub enable_images: Option<bool>,
    pub exclude_active_sessions: Option<bool>,
}

impl Client {
    pub async fn get_resume_items(
        &self,
        query: Option<ResumeItemsQuery>,
    ) -> Result<PaginatedResult<Item>> {
        self.get("UserItems/Resume", query).await
    }
}

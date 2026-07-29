use std::collections::HashMap;

use serde::Deserialize;

use crate::models::items::{
    UserData,
    enums::{CollectionType, ItemKind},
    media::ImageBlurHashes,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Library {
    pub id: String,
    pub name: String,
    pub server_id: String,
    pub etag: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: ItemKind,
    pub collection_type: Option<CollectionType>,
    pub sort_name: Option<String>,
    pub parent_id: Option<String>,
    pub child_count: Option<i32>,
    pub image_tags: HashMap<String, String>,
    pub backdrop_image_tags: Vec<String>,
    pub image_blur_hashes: ImageBlurHashes,
    pub primary_image_aspect_ratio: Option<f64>,
    pub display_preferences_id: Option<String>,
    pub user_data: UserData,
}

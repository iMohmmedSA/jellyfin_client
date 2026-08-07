use bytes::Bytes;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Serialize;
use url::Url;

use crate::{
    Client,
    error::Result,
    models::items::query_enums::{ImageFormat, ImageType},
};

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageQuery {
    pub tag: Option<String>,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub quality: Option<i32>,
    pub fill_width: Option<i32>,
    pub fill_height: Option<i32>,
    pub format: Option<ImageFormat>,
    pub percent_played: Option<f64>,
    pub unplayed_count: Option<i32>,
    pub blur: Option<i32>,
    pub background_color: Option<String>,
    pub foreground_layer: Option<String>,
    pub image_index: Option<i32>,
}

impl Client {
    pub fn image_url(
        &self,
        item_id: &str,
        image_type: ImageType,
        query: Option<&ImageQuery>,
    ) -> Result<Url> {
        self.build_url(&item_image_path(item_id, &image_type), query)
    }

    pub async fn get_item_image(
        &self,
        item_id: &str,
        image_type: ImageType,
        query: Option<ImageQuery>,
    ) -> Result<Bytes> {
        self.get_bytes(&item_image_path(item_id, &image_type), query)
            .await
    }
}

fn item_image_path(item_id: &str, image_type: &ImageType) -> String {
    format!(
        "Items/{}/Images/{image_type}",
        utf8_percent_encode(item_id, NON_ALPHANUMERIC)
    )
}

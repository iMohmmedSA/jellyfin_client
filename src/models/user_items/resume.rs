use serde::Deserialize;

use crate::models::items::Item;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resume {
    pub items: Vec<Item>,
    pub total_record_count: i32,
    pub start_index: i32,
}

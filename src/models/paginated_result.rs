use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_record_count: i32,
    pub start_index: i32,
}

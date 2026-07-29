use serde::Serialize;

use crate::{
    Client,
    error::Result,
    models::{PaginatedResult, items::enums::CollectionType, user_views::Library},
};

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserViewsQuery {
    pub user_id: Option<String>,
    pub include_external_content: Option<bool>,
    #[serde(serialize_with = "crate::serde_util::comma_separated")]
    pub preset_views: Option<Vec<CollectionType>>,
    pub include_hidden: Option<bool>,
}

impl Client {
    pub async fn get_libraries(
        &self,
        query: Option<UserViewsQuery>,
    ) -> Result<PaginatedResult<Library>> {
        self.get("UserViews", query).await
    }
}

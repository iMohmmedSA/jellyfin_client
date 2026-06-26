use crate::{client::Client, error::Result, models::system::PublicInfo};

impl Client {
    pub async fn public_info(&self) -> Result<PublicInfo> {
        self.get("/System/Info/Public", None::<()>).await
    }
}

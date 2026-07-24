use crate::{client::Client, error::Result, models::system::Endpoint};

impl Client {
    pub async fn endpoint(&self) -> Result<Endpoint> {
        self.get("System/Endpoint", None::<()>).await
    }
}

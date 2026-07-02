use serde::Serialize;

use crate::{client::Client, error::Result, models::users::Authenticate};

#[derive(Debug, Serialize)]
pub struct Credentials {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Pw")]
    pub password: String,
}

impl Credentials {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl Client {
    pub async fn authenticate(&self, credentials: Credentials) -> Result<Authenticate> {
        let auth: Authenticate = self
            .post("Users/AuthenticateByName", credentials.into(), None::<()>)
            .await?;

        if let Some(token) = auth.access_token.as_deref() {
            self.token(token).await;
        }

        Ok(auth)
    }
}

use std::sync::Arc;

use tokio::sync::RwLock;
use url::Url;

use crate::headers::TOKEN;

#[derive(Debug)]
pub struct Inner {
    pub(crate) reqwest: reqwest::Client,

    pub(crate) base: Url,

    // Auth
    pub(crate) auth_base: Arc<str>,
    pub(crate) auth: RwLock<Option<Arc<str>>>,
}

impl Inner {
    pub(crate) async fn auth(&self) -> Arc<str> {
        match *self.auth.read().await {
            Some(ref a) => a.clone(),
            None => self.auth_base.clone(),
        }
    }

    pub(crate) async fn set_token(&self, token: Option<&str>) {
        *self.auth.write().await =
            token.map(|t| Arc::from(format!("{}, {}=\"{}\"", self.auth_base, TOKEN, t)));
    }
}

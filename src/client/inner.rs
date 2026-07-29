use std::sync::Arc;

use tokio::sync::RwLock;
use url::Url;

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
}

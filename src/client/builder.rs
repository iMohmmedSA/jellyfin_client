use std::sync::Arc;

use url::Url;

use crate::client::{Client, inner::Inner};

#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base: Url,
}

impl ClientBuilder {
    pub fn new(base: Url) -> ClientBuilder {
        ClientBuilder { base }
    }

    pub fn build(self) -> Client {
        let reqwest = reqwest::Client::new();

        let inner = Arc::new(Inner {
            reqwest,
            base: self.base,
        });

        Client { inner }
    }
}

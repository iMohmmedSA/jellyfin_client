pub mod builder;
pub mod inner;

pub use builder::ClientBuilder;
use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use std::sync::Arc;

use crate::{
    client::inner::Inner,
    error::{Error, Result},
    headers::AUTHORIZATION,
};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

impl Client {
    pub fn builder(base: Url) -> ClientBuilder {
        ClientBuilder::new(base)
    }

    pub(crate) async fn get<T, Q>(&self, path: &str, query: Option<Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(Method::GET, path, None::<()>, query).await
    }

    pub(crate) async fn request<T, B, Q>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
        Q: Serialize,
    {
        let url = self.inner.base.join(path)?;

        let mut req = self.inner.reqwest.request(method, url);

        if let Some(query) = query {
            req = req.query(&query);
        }

        req = req.header(AUTHORIZATION, &*self.inner.auth().await);

        if let Some(body) = body {
            req = req.json(&body);
        }

        let res = req.send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let message = res
                .text()
                .await
                .unwrap_or_else(|e| format!("Failed to read error body: {e}"));
            return Err(Error::Api { status, message });
        }

        Ok(res.json().await?)
    }
}

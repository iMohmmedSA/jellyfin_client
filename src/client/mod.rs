pub mod inner;

use reqwest::{
    Method,
    header::{ACCEPT, HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;
use url::Url;

use std::sync::Arc;

use crate::{
    client::inner::Inner,
    error::{Error, Result},
    headers::{AUTHORIZATION, CLIENT, DEVICE, DEVICE_ID, TOKEN, VERSION},
};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

impl Client {
    pub fn new(
        base: Url,
        client: impl Into<String>,
        device: impl Into<String>,
        device_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Client> {
        Self::new_inner(
            base,
            client.into(),
            device.into(),
            device_id.into(),
            version.into(),
            None,
        )
    }

    pub fn new_with_token(
        base: Url,
        client: impl Into<String>,
        device: impl Into<String>,
        device_id: impl Into<String>,
        version: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Client> {
        Self::new_inner(
            base,
            client.into(),
            device.into(),
            device_id.into(),
            version.into(),
            Some(token.into()),
        )
    }

    fn new_inner(
        base: Url,
        client: String,
        device: String,
        device_id: String,
        version: String,
        token: Option<String>,
    ) -> Result<Client> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let reqwest = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let auth_base = format!(
            "MediaBrowser {}=\"{}\", {}=\"{}\", {}=\"{}\", {}=\"{}\"",
            CLIENT, client, DEVICE, device, DEVICE_ID, device_id, VERSION, version,
        );

        let auth = match token {
            Some(token) => format!("{auth_base}, {TOKEN}=\"{token}\""),
            None => auth_base.clone(),
        };

        let auth_base: Arc<str> = Arc::from(auth_base);
        let auth: Arc<str> = Arc::from(auth);

        let inner = Arc::new(Inner {
            reqwest,
            base,
            auth_base,
            auth: RwLock::from(Some(auth)),
        });

        Ok(Self { inner })
    }

    pub async fn token(&self, token: impl Into<String>) {
        let base = self.inner.auth_base.clone();
        let token = token.into();

        let mut auth = self.inner.auth.write().await;
        *auth = Some(Arc::<str>::from(format!("{base}, {TOKEN}=\"{token}\"")));
    }

    pub(crate) async fn get<T, Q>(&self, path: &str, query: Option<Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.request(Method::GET, path, None::<()>, query).await
    }

    pub(crate) async fn post<T, B, Q>(
        &self,
        path: &str,
        body: Option<B>,
        query: Option<Q>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
        Q: Serialize,
    {
        self.request(Method::POST, path, body, query).await
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

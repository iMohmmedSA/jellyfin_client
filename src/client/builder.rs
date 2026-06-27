use std::sync::Arc;

use tokio::sync::RwLock;
use url::Url;

use crate::{
    client::{Client, inner::Inner},
    headers::{CLIENT, DEVICE, DEVICE_ID, TOKEN, VERSION},
};

#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base: Url,
    token: Option<String>,
    client: Option<String>,
    device: Option<String>,
    device_id: Option<String>,
    version: Option<String>,
}

impl ClientBuilder {
    pub fn new(base: Url) -> ClientBuilder {
        ClientBuilder {
            base,
            token: None,
            client: None,
            device: None,
            device_id: None,
            version: None,
        }
    }

    pub fn client(mut self, client: impl Into<String>) -> Self {
        self.client = Some(client.into());
        self
    }

    pub fn device(mut self, device: impl Into<String>) -> Self {
        self.device = Some(device.into());
        self
    }

    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn mabey_token(mut self, token: Option<impl Into<String>>) -> Self {
        self.token = token.map(|t| t.into());
        self
    }

    pub fn build(self) -> Client {
        let reqwest = reqwest::Client::new();

        let mut parts: Vec<String> = Vec::new();

        let mut insert_part = |name: &'static str, value: Option<String>| {
            if let Some(v) = value {
                parts.push(format!("{}=\"{}\"", name, v));
            }
        };

        insert_part(CLIENT, self.client);
        insert_part(DEVICE, self.device);
        insert_part(DEVICE_ID, self.device_id);
        insert_part(VERSION, self.version);

        let auth_base = Arc::from(format!("MediaBrowser {}", parts.join(", ")));
        let auth = RwLock::new(
            self.token
                .map(|t| Arc::from(format!("{}, {}=\"{}\"", auth_base, TOKEN, t))),
        );

        let inner = Arc::new(Inner {
            reqwest,
            base: self.base,
            auth_base,
            auth,
        });

        Client { inner }
    }
}

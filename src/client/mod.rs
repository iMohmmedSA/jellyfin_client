pub mod builder;
pub mod inner;

pub use builder::ClientBuilder;
use url::Url;

use std::sync::Arc;

use crate::client::inner::Inner;

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

impl Client {
    pub fn builder(base: Url) -> ClientBuilder {
        ClientBuilder::new(base)
    }
}

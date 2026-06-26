use url::Url;

#[derive(Debug, Clone)]
pub struct Inner {
    pub(crate) reqwest: reqwest::Client,

    pub(crate) base: Url,
}

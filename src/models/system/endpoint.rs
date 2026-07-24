use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    pub is_local: bool,
    pub is_in_network: bool,
}

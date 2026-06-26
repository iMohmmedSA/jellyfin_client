use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublicInfo {
    pub local_address: Option<String>,
    pub server_name: Option<String>,
    pub version: Option<String>,
    pub product_name: Option<String>,
    pub id: Option<String>,
    pub startup_wizard_completed: Option<bool>,
}

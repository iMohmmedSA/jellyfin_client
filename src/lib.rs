pub mod client;
pub mod error;

pub(crate) mod headers {
    pub const AUTHORIZATION: &str = "Authorization";
    pub const CLIENT: &str = "Client";
    pub const DEVICE: &str = "Device";
    pub const DEVICE_ID: &str = "DeviceId";
    pub const VERSION: &str = "Version";
    pub const TOKEN: &str = "Token";

    pub const RETRY_AFTER: &str = "Retry-After";
    pub const MESSAGE: &str = "Message";
}

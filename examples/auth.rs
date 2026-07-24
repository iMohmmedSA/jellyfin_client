use jellyfin_client::{Client, api::users::authenticate::Credentials};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("http://localhost").unwrap();
    let client = Client::new(
        url,
        "YourAppName",
        "DeviceName",
        "RandomIDForEachClient",
        "0.1.0",
    )
    .unwrap();

    let login = Credentials::new("Username", "Password");
    println!("{:#?}", client.authenticate(login).await);
}

use jellyfin_client::{api::users::authenticate::Credentials, client::Client};
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

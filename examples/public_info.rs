use jellyfin_client::Client;
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

    println!("{:?}", client.public_info().await);
}

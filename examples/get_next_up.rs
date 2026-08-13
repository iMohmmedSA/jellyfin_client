use jellyfin_client::Client;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("http://localhost").unwrap();
    let client = Client::new_with_token(
        url,
        "YourAppName",
        "DeviceName",
        "RandomIDForEachClient",
        "0.1.0",
        "TOKEN",
    )
    .unwrap();

    println!("{:?}", client.get_next_up(None).await);
}

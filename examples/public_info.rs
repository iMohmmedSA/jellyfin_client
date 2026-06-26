use jellyfin_client::client::Client;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("http://localhost").unwrap();
    let client = Client::builder(url).build();
    println!("{:?}", client.public_info().await);
}

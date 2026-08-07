use jellyfin_client::{Client, models::items::query_enums::ImageType};
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

    let libraries = client.get_libraries(None).await.unwrap();
    let library = libraries.items.first().expect("no libraries found");

    let image_url = client
        .image_url(&library.id, ImageType::Primary, None)
        .unwrap();
    println!("Image URL: {image_url}");

    let bytes = client
        .get_item_image(&library.id, ImageType::Primary, None)
        .await
        .unwrap();

    std::fs::write("library_primary.jpg", &bytes).unwrap();
    println!("Saved image to library_primary.jpg");
}

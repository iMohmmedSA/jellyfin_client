use serde::Deserialize;

use crate::models::items::media::ImageBlurHashes;

#[derive(Debug, Deserialize)]
pub enum PersonKind {
    Unknown,
    Actor,
    Director,
    Composer,
    Writer,
    GuestStar,
    Producer,
    Conductor,
    Lyricist,
    Arranger,
    Engineer,
    Mixer,
    Remixer,
    Creator,
    Artist,
    AlbumArtist,
    Author,
    Illustrator,
    Penciller,
    Inker,
    Colorist,
    Letterer,
    CoverArtist,
    Editor,
    Translator,
    Narrator,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PersonCredit {
    pub name: Option<String>,
    pub id: String,
    pub role: Option<String>,
    #[serde(rename = "Type")]
    pub kind: PersonKind,
    pub primary_image_tag: Option<String>,
    pub image_blur_hashes: Option<ImageBlurHashes>,
}

use std::collections::HashMap;

use serde::Deserialize;

use crate::models::items::{
    enums::{
        Audio, ChannelType, CollectionType, DayOfWeek, ExtraType, ImageOrientation, ItemKind,
        LocationType, MediaType, MetadataField, PlayAccess,
    },
    media::{
        ChapterInfo, ImageBlurHashes, IsoType, MediaSourceInfo, MediaStream, TrickplayInfo,
        Video3DFormat, VideoType,
    },
    person::PersonCredit,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub name: Option<String>,
    pub original_title: Option<String>,
    pub server_id: Option<String>,
    pub id: Option<String>,
    pub etag: Option<String>,
    pub source_type: Option<String>,
    pub playlist_item_id: Option<String>,
    pub date_created: Option<String>,
    pub date_last_media_added: Option<String>,
    pub extra_type: Option<ExtraType>,
    pub airs_before_season_number: Option<i32>,
    pub airs_after_season_number: Option<i32>,
    pub airs_before_episode_number: Option<i32>,
    pub can_delete: Option<bool>,
    pub can_download: Option<bool>,
    pub has_lyrics: Option<bool>,
    pub has_subtitles: Option<bool>,
    pub preferred_metadata_language: Option<String>,
    pub preferred_metadata_country_code: Option<String>,
    pub container: Option<String>,
    pub sort_name: Option<String>,
    pub forced_sort_name: Option<String>,
    #[serde(rename = "Video3DFormat")]
    pub video_3d_format: Option<Video3DFormat>,
    pub premiere_date: Option<String>,
    pub external_urls: Option<Vec<NameUrlPair>>,
    pub media_sources: Option<Vec<MediaSourceInfo>>,
    pub critic_rating: Option<f32>,
    pub production_locations: Option<Vec<String>>,
    pub path: Option<String>,
    pub enable_media_source_display: Option<bool>,
    pub official_rating: Option<String>,
    pub custom_rating: Option<String>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub overview: Option<String>,
    pub taglines: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub community_rating: Option<f32>,
    pub cumulative_run_time_ticks: Option<i64>,
    pub run_time_ticks: Option<i64>,
    pub play_access: Option<PlayAccess>,
    pub aspect_ratio: Option<String>,
    pub production_year: Option<i32>,
    pub is_place_holder: Option<bool>,
    pub number: Option<String>,
    pub channel_number: Option<String>,
    pub index_number: Option<i32>,
    pub index_number_end: Option<i32>,
    pub parent_index_number: Option<i32>,
    pub remote_trailers: Option<Vec<NameUrlPair>>,
    pub provider_ids: Option<HashMap<String, String>>,
    #[serde(rename = "IsHD")]
    pub is_hd: Option<bool>,
    pub is_folder: Option<bool>,
    pub parent_id: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: Option<ItemKind>,
    pub people: Option<Vec<PersonCredit>>,
    pub studios: Option<Vec<NameGuidPair>>,
    pub genre_items: Option<Vec<NameGuidPair>>,
    #[serde(flatten)]
    pub parent_images: ParentImages,
    pub local_trailer_count: Option<i32>,
    pub user_data: Option<UserData>,
    pub recursive_item_count: Option<i32>,
    pub child_count: Option<i32>,
    pub series_name: Option<String>,
    pub series_id: Option<String>,
    pub season_id: Option<String>,
    pub special_feature_count: Option<i32>,
    pub display_preferences_id: Option<String>,
    pub status: Option<String>,
    pub air_time: Option<String>,
    pub air_days: Option<Vec<DayOfWeek>>,
    pub tags: Option<Vec<String>>,
    pub primary_image_aspect_ratio: Option<f64>,
    pub artists: Option<Vec<String>>,
    pub artist_items: Option<Vec<NameGuidPair>>,
    pub album: Option<String>,
    pub collection_type: Option<CollectionType>,
    pub display_order: Option<String>,
    pub album_id: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Option<Vec<NameGuidPair>>,
    pub season_name: Option<String>,
    pub media_streams: Option<Vec<MediaStream>>,
    pub video_type: Option<VideoType>,
    pub part_count: Option<i32>,
    pub media_source_count: Option<i32>,
    pub image_tags: Option<HashMap<String, String>>,
    pub backdrop_image_tags: Option<Vec<String>>,
    pub screenshot_image_tags: Option<Vec<String>>,
    pub image_blur_hashes: Option<ImageBlurHashes>,
    pub series_studio: Option<String>,
    pub chapters: Option<Vec<ChapterInfo>>,
    pub trickplay: Option<HashMap<String, HashMap<String, TrickplayInfo>>>,
    pub location_type: Option<LocationType>,
    pub iso_type: Option<IsoType>,
    pub media_type: Option<MediaType>,
    pub end_date: Option<String>,
    pub locked_fields: Option<Vec<MetadataField>>,
    pub trailer_count: Option<i32>,
    pub movie_count: Option<i32>,
    pub series_count: Option<i32>,
    pub program_count: Option<i32>,
    pub episode_count: Option<i32>,
    pub song_count: Option<i32>,
    pub album_count: Option<i32>,
    pub artist_count: Option<i32>,
    pub music_video_count: Option<i32>,
    pub lock_data: Option<bool>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub software: Option<String>,
    pub exposure_time: Option<f64>,
    pub focal_length: Option<f64>,
    pub image_orientation: Option<ImageOrientation>,
    pub aperture: Option<f64>,
    pub shutter_speed: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub iso_speed_rating: Option<i32>,
    pub series_timer_id: Option<String>,
    pub program_id: Option<String>,
    pub start_date: Option<String>,
    pub completion_percentage: Option<f64>,
    pub is_repeat: Option<bool>,
    pub episode_title: Option<String>,
    pub channel_type: Option<ChannelType>,
    #[serde(rename = "Audio")]
    pub audio_format: Option<Audio>,
    pub is_movie: Option<bool>,
    pub is_sports: Option<bool>,
    pub is_series: Option<bool>,
    pub is_live: Option<bool>,
    pub is_news: Option<bool>,
    pub is_kids: Option<bool>,
    pub is_premiere: Option<bool>,
    pub timer_id: Option<String>,
    pub normalization_gain: Option<f32>,
    pub album_normalization_gain: Option<f32>,
    pub current_program: Option<Box<Item>>,
    pub original_language: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParentImages {
    #[serde(rename = "ParentLogoItemId")]
    pub logo_item_id: Option<String>,
    #[serde(rename = "ParentLogoImageTag")]
    pub logo_image_tag: Option<String>,
    #[serde(rename = "ParentBackdropItemId")]
    pub backdrop_item_id: Option<String>,
    #[serde(rename = "ParentBackdropImageTags")]
    pub backdrop_image_tags: Option<Vec<String>>,
    #[serde(rename = "ParentArtItemId")]
    pub art_item_id: Option<String>,
    #[serde(rename = "ParentArtImageTag")]
    pub art_image_tag: Option<String>,
    #[serde(rename = "ParentThumbItemId")]
    pub thumb_item_id: Option<String>,
    #[serde(rename = "ParentThumbImageTag")]
    pub thumb_image_tag: Option<String>,
    #[serde(rename = "ParentPrimaryImageItemId")]
    pub primary_image_item_id: Option<String>,
    #[serde(rename = "ParentPrimaryImageTag")]
    pub primary_image_tag: Option<String>,
    pub series_thumb_image_tag: Option<String>,
    pub series_primary_image_tag: Option<String>,
    pub album_primary_image_tag: Option<String>,
    pub channel_primary_image_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameUrlPair {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameGuidPair {
    pub name: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    pub rating: Option<f64>,
    pub played_percentage: Option<f64>,
    pub unplayed_item_count: Option<i32>,
    pub playback_position_ticks: i64,
    pub play_count: i32,
    pub is_favorite: bool,
    pub likes: Option<bool>,
    pub last_played_date: Option<String>,
    pub played: bool,
    pub key: String,
    pub item_id: Option<String>,
}

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum VideoType {
    VideoFile,
    Iso,
    Dvd,
    BluRay,
}

#[derive(Debug, Deserialize)]
pub enum IsoType {
    Dvd,
    BluRay,
}

#[derive(Debug, Deserialize)]
pub enum Video3DFormat {
    HalfSideBySide,
    FullSideBySide,
    FullTopAndBottom,
    HalfTopAndBottom,
    #[serde(rename = "MVC")]
    Mvc,
}

#[derive(Debug, Deserialize)]
pub enum MediaProtocol {
    File,
    Http,
    Rtmp,
    Rtsp,
    Udp,
    Rtp,
    Ftp,
}

#[derive(Debug, Deserialize)]
pub enum MediaSourceType {
    Default,
    Grouping,
    Placeholder,
}

#[derive(Debug, Deserialize)]
pub enum TransportStreamTimestamp {
    None,
    Zero,
    Valid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaStreamProtocol {
    Http,
    Hls,
}

#[derive(Debug, Deserialize)]
pub enum VideoRange {
    Unknown,
    #[serde(rename = "SDR")]
    Sdr,
    #[serde(rename = "HDR")]
    Hdr,
}

#[derive(Debug, Deserialize)]
pub enum VideoRangeType {
    Unknown,
    #[serde(rename = "SDR")]
    Sdr,
    #[serde(rename = "HDR10")]
    Hdr10,
    #[serde(rename = "HLG")]
    Hlg,
    #[serde(rename = "DOVI")]
    Dovi,
    #[serde(rename = "DOVIWithHDR10")]
    DoviWithHdr10,
    #[serde(rename = "DOVIWithHLG")]
    DoviWithHlg,
    #[serde(rename = "DOVIWithSDR")]
    DoviWithSdr,
    #[serde(rename = "DOVIWithEL")]
    DoviWithEl,
    #[serde(rename = "DOVIWithHDR10Plus")]
    DoviWithHdr10Plus,
    #[serde(rename = "DOVIWithELHDR10Plus")]
    DoviWithElHdr10Plus,
    #[serde(rename = "DOVIInvalid")]
    DoviInvalid,
    #[serde(rename = "HDR10Plus")]
    Hdr10Plus,
}

#[derive(Debug, Deserialize)]
pub enum AudioSpatialFormat {
    None,
    DolbyAtmos,
    #[serde(rename = "DTSX")]
    Dtsx,
}

#[derive(Debug, Deserialize)]
pub enum MediaStreamType {
    Audio,
    Video,
    Subtitle,
    EmbeddedImage,
    Data,
    Lyric,
}

#[derive(Debug, Deserialize)]
pub enum SubtitleDeliveryMethod {
    Encode,
    Embed,
    External,
    Hls,
    Drop,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaAttachment {
    pub codec: Option<String>,
    pub codec_tag: Option<String>,
    pub comment: Option<String>,
    pub index: i32,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub delivery_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaStream {
    pub codec: Option<String>,
    pub codec_tag: Option<String>,
    pub language: Option<String>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub color_transfer: Option<String>,
    pub color_primaries: Option<String>,
    pub dv_version_major: Option<i32>,
    pub dv_version_minor: Option<i32>,
    pub dv_profile: Option<i32>,
    pub dv_level: Option<i32>,
    pub rpu_present_flag: Option<i32>,
    pub el_present_flag: Option<i32>,
    pub bl_present_flag: Option<i32>,
    pub dv_bl_signal_compatibility_id: Option<i32>,
    pub rotation: Option<i32>,
    pub comment: Option<String>,
    pub time_base: Option<String>,
    pub codec_time_base: Option<String>,
    pub title: Option<String>,
    pub hdr10_plus_present_flag: Option<bool>,
    pub video_range: VideoRange,
    pub video_range_type: VideoRangeType,
    pub video_do_vi_title: Option<String>,
    pub audio_spatial_format: AudioSpatialFormat,
    pub localized_undefined: Option<String>,
    pub localized_default: Option<String>,
    pub localized_forced: Option<String>,
    pub localized_external: Option<String>,
    pub localized_hearing_impaired: Option<String>,
    pub localized_language: Option<String>,
    pub localized_original: Option<String>,
    pub display_title: Option<String>,
    pub nal_length_size: Option<String>,
    pub is_interlaced: bool,
    #[serde(rename = "IsAVC")]
    pub is_avc: Option<bool>,
    pub channel_layout: Option<String>,
    pub bit_rate: Option<i32>,
    pub bit_depth: Option<i32>,
    pub ref_frames: Option<i32>,
    pub packet_length: Option<i32>,
    pub channels: Option<i32>,
    pub sample_rate: Option<i32>,
    pub is_default: bool,
    pub is_forced: bool,
    pub is_hearing_impaired: bool,
    pub is_original: bool,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub average_frame_rate: Option<f32>,
    pub real_frame_rate: Option<f32>,
    pub reference_frame_rate: Option<f32>,
    pub profile: Option<String>,
    #[serde(rename = "Type")]
    pub stream_type: MediaStreamType,
    pub aspect_ratio: Option<String>,
    pub index: i32,
    pub score: Option<i32>,
    pub is_external: bool,
    pub delivery_method: Option<SubtitleDeliveryMethod>,
    pub delivery_url: Option<String>,
    pub is_external_url: Option<bool>,
    pub is_text_subtitle_stream: bool,
    pub supports_external_stream: bool,
    pub path: Option<String>,
    pub pixel_format: Option<String>,
    pub level: Option<f64>,
    pub is_anamorphic: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSourceInfo {
    pub protocol: MediaProtocol,
    pub id: Option<String>,
    pub path: Option<String>,
    pub encoder_path: Option<String>,
    pub encoder_protocol: Option<MediaProtocol>,
    #[serde(rename = "Type")]
    pub source_type: MediaSourceType,
    pub container: Option<String>,
    pub size: Option<i64>,
    pub name: Option<String>,
    pub is_remote: bool,
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    pub run_time_ticks: Option<i64>,
    pub read_at_native_framerate: bool,
    pub ignore_dts: bool,
    pub ignore_index: bool,
    pub gen_pts_input: bool,
    pub supports_transcoding: bool,
    pub supports_direct_stream: bool,
    pub supports_direct_play: bool,
    pub is_infinite_stream: bool,
    pub use_most_compatible_transcoding_profile: bool,
    pub requires_opening: bool,
    pub open_token: Option<String>,
    pub requires_closing: bool,
    pub live_stream_id: Option<String>,
    pub buffer_ms: Option<i32>,
    pub requires_looping: bool,
    pub supports_probing: bool,
    pub video_type: Option<VideoType>,
    pub iso_type: Option<IsoType>,
    #[serde(rename = "Video3DFormat")]
    pub video_3d_format: Option<Video3DFormat>,
    pub media_streams: Option<Vec<MediaStream>>,
    pub media_attachments: Option<Vec<MediaAttachment>>,
    pub formats: Option<Vec<String>>,
    pub bitrate: Option<i32>,
    pub fallback_max_streaming_bitrate: Option<i32>,
    pub timestamp: Option<TransportStreamTimestamp>,
    pub required_http_headers: Option<HashMap<String, String>>,
    pub transcoding_url: Option<String>,
    pub transcoding_sub_protocol: MediaStreamProtocol,
    pub transcoding_container: Option<String>,
    pub analyze_duration_ms: Option<i32>,
    pub default_audio_stream_index: Option<i32>,
    pub default_subtitle_stream_index: Option<i32>,
    pub has_segments: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrickplayInfo {
    pub width: i32,
    pub height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub thumbnail_count: i32,
    pub interval: i32,
    pub bandwidth: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChapterInfo {
    pub start_position_ticks: i64,
    pub name: Option<String>,
    pub image_path: Option<String>,
    pub image_date_modified: String,
    pub image_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageBlurHashes {
    pub primary: Option<HashMap<String, String>>,
    pub art: Option<HashMap<String, String>>,
    pub backdrop: Option<HashMap<String, String>>,
    pub banner: Option<HashMap<String, String>>,
    pub logo: Option<HashMap<String, String>>,
    pub thumb: Option<HashMap<String, String>>,
    pub disc: Option<HashMap<String, String>>,
    #[serde(rename = "Box")]
    pub box_: Option<HashMap<String, String>>,
    pub screenshot: Option<HashMap<String, String>>,
    pub menu: Option<HashMap<String, String>>,
    pub chapter: Option<HashMap<String, String>>,
    #[serde(rename = "BoxRear")]
    pub box_rear: Option<HashMap<String, String>>,
    pub profile: Option<HashMap<String, String>>,
}

use std::collections::HashMap;

use serde::Deserialize;

// -- Enums --

#[derive(Debug, Deserialize)]
pub enum GeneralCommandType {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    PreviousLetter,
    NextLetter,
    ToggleOsd,
    ToggleContextMenu,
    Select,
    Back,
    TakeScreenshot,
    SendKey,
    SendString,
    GoHome,
    GoToSettings,
    VolumeUp,
    VolumeDown,
    Mute,
    Unmute,
    ToggleMute,
    SetVolume,
    SetAudioStreamIndex,
    SetSubtitleStreamIndex,
    ToggleFullscreen,
    DisplayContent,
    GoToSearch,
    DisplayMessage,
    SetRepeatMode,
    ChannelUp,
    ChannelDown,
    Guide,
    ToggleStats,
    PlayMediaSource,
    PlayTrailers,
    SetShuffleQueue,
    PlayState,
    PlayNext,
    ToggleOsdMenu,
    Play,
    SetMaxStreamingBitrate,
    SetPlaybackOrder,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HardwareAccelerationType {
    None,
    Amf,
    Qsv,
    Nvenc,
    V4l2m2m,
    Vaapi,
    Videotoolbox,
    Rkmpp,
}

#[derive(Debug, Deserialize)]
pub enum TranscodeReason {
    ContainerNotSupported,
    VideoCodecNotSupported,
    AudioCodecNotSupported,
    SubtitleCodecNotSupported,
    AudioIsExternal,
    SecondaryAudioNotSupported,
    VideoProfileNotSupported,
    VideoLevelNotSupported,
    VideoResolutionNotSupported,
    VideoBitDepthNotSupported,
    VideoFramerateNotSupported,
    RefFramesNotSupported,
    AnamorphicVideoNotSupported,
    InterlacedVideoNotSupported,
    AudioChannelsNotSupported,
    AudioProfileNotSupported,
    AudioSampleRateNotSupported,
    AudioBitDepthNotSupported,
    ContainerBitrateExceedsLimit,
    VideoBitrateNotSupported,
    AudioBitrateNotSupported,
    UnknownVideoStreamInfo,
    UnknownAudioStreamInfo,
    DirectPlayError,
    VideoRangeTypeNotSupported,
    VideoCodecTagNotSupported,
    StreamCountExceedsLimit,
    VideoRotationNotSupported,
}

#[derive(Debug, Deserialize)]
pub enum PlayMethod {
    Transcode,
    DirectStream,
    DirectPlay,
}

#[derive(Debug, Deserialize)]
pub enum RepeatMode {
    RepeatNone,
    RepeatAll,
    RepeatOne,
}

#[derive(Debug, Deserialize)]
pub enum PlaybackOrder {
    Default,
    Shuffle,
}

#[derive(Debug, Deserialize)]
pub enum MediaType {
    Unknown,
    Video,
    Audio,
    Photo,
    Book,
}

#[derive(Debug, Deserialize)]
pub enum SubtitlePlaybackMode {
    Default,
    Always,
    OnlyForced,
    None,
    Smart,
}

#[derive(Debug, Deserialize)]
pub enum DynamicDayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Everyday,
    Weekday,
    Weekend,
}

#[derive(Debug, Deserialize)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Deserialize)]
pub enum UnratedItem {
    Movie,
    Trailer,
    Series,
    Music,
    Book,
    LiveTvChannel,
    LiveTvProgram,
    ChannelContent,
    Other,
}

#[derive(Debug, Deserialize)]
pub enum SyncPlayUserAccessType {
    CreateAndJoinGroups,
    JoinGroups,
    None,
}

#[derive(Debug, Deserialize)]
pub enum Audio {
    Mono,
    Stereo,
    Dolby,
    DolbyDigital,
    Thx,
    Atmos,
}

#[derive(Debug, Deserialize)]
pub enum ChannelType {
    Tv,
    Radio,
}

#[derive(Debug, Deserialize)]
pub enum ImageOrientation {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    LeftTop,
    RightTop,
    RightBottom,
    LeftBottom,
}

#[derive(Debug, Deserialize)]
pub enum MetadataField {
    Cast,
    Genres,
    ProductionLocations,
    Studios,
    Tags,
    Name,
    Overview,
    Runtime,
    OfficialRating,
}

#[derive(Debug, Deserialize)]
pub enum IsoType {
    Dvd,
    BluRay,
}

#[derive(Debug, Deserialize)]
pub enum LocationType {
    FileSystem,
    Remote,
    Virtual,
    Offline,
}

#[derive(Debug, Deserialize)]
pub enum ExtraType {
    Unknown,
    Clip,
    Trailer,
    BehindTheScenes,
    DeletedScene,
    Interview,
    Scene,
    Sample,
    ThemeSong,
    ThemeVideo,
    Featurette,
    Short,
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
pub enum PlayAccess {
    Full,
    None,
}

#[derive(Debug, Deserialize)]
pub enum BaseItemKind {
    AggregateFolder,
    Audio,
    AudioBook,
    BasePluginFolder,
    Book,
    BoxSet,
    Channel,
    ChannelFolderItem,
    CollectionFolder,
    Episode,
    Folder,
    Genre,
    ManualPlaylistsFolder,
    Movie,
    LiveTvChannel,
    LiveTvProgram,
    MusicAlbum,
    MusicArtist,
    MusicGenre,
    MusicVideo,
    Person,
    Photo,
    PhotoAlbum,
    Playlist,
    PlaylistsFolder,
    Program,
    Recording,
    Season,
    Series,
    Studio,
    Trailer,
    TvChannel,
    TvProgram,
    UserRootFolder,
    UserView,
    Video,
    Year,
}

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
pub enum VideoType {
    VideoFile,
    Iso,
    Dvd,
    BluRay,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CollectionType {
    Unknown,
    Movies,
    TvShows,
    Music,
    MusicVideos,
    Trailers,
    HomeVideos,
    BoxSets,
    Books,
    Photos,
    LiveTv,
    Playlists,
    Folders,
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

// -- Structs --

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueueItem {
    pub id: String,
    pub playlist_item_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TranscodingInfo {
    pub audio_codec: Option<String>,
    pub video_codec: Option<String>,
    pub container: Option<String>,
    pub is_video_direct: bool,
    pub is_audio_direct: bool,
    pub bitrate: Option<i32>,
    pub framerate: Option<f32>,
    pub completion_percentage: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub audio_channels: Option<i32>,
    pub hardware_acceleration_type: Option<HardwareAccelerationType>,
    pub transcode_reasons: Vec<TranscodeReason>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerStateInfo {
    pub position_ticks: Option<i64>,
    pub can_seek: bool,
    pub is_paused: bool,
    pub is_muted: bool,
    pub volume_level: Option<i32>,
    pub audio_stream_index: Option<i32>,
    pub subtitle_stream_index: Option<i32>,
    pub media_source_id: Option<String>,
    pub play_method: Option<PlayMethod>,
    pub repeat_mode: RepeatMode,
    pub playback_order: PlaybackOrder,
    pub live_stream_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionUserInfo {
    pub user_id: String,
    pub user_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessSchedule {
    pub id: i32,
    pub user_id: String,
    pub day_of_week: DynamicDayOfWeek,
    pub start_hour: f64,
    pub end_hour: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserConfiguration {
    pub audio_language_preference: Option<String>,
    pub play_default_audio_track: bool,
    pub subtitle_language_preference: Option<String>,
    pub display_missing_episodes: bool,
    pub grouped_folders: Vec<String>,
    pub subtitle_mode: SubtitlePlaybackMode,
    pub display_collections_view: bool,
    pub enable_local_password: bool,
    pub ordered_views: Vec<String>,
    pub latest_items_excludes: Vec<String>,
    pub my_media_excludes: Vec<String>,
    pub hide_played_in_latest: bool,
    pub remember_audio_selections: bool,
    pub remember_subtitle_selections: bool,
    pub enable_next_episode_auto_play: bool,
    pub cast_receiver_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPolicy {
    pub is_administrator: bool,
    pub is_hidden: bool,
    pub enable_collection_management: bool,
    pub enable_subtitle_management: bool,
    pub enable_lyric_management: bool,
    pub is_disabled: bool,
    pub max_parental_rating: Option<i32>,
    pub max_parental_sub_rating: Option<i32>,
    pub blocked_tags: Option<Vec<String>>,
    pub allowed_tags: Option<Vec<String>>,
    pub enable_user_preference_access: bool,
    pub access_schedules: Option<Vec<AccessSchedule>>,
    pub block_unrated_items: Option<Vec<UnratedItem>>,
    pub enable_remote_control_of_other_users: bool,
    pub enable_shared_device_control: bool,
    pub enable_remote_access: bool,
    pub enable_live_tv_management: bool,
    pub enable_live_tv_access: bool,
    pub enable_media_playback: bool,
    pub enable_audio_playback_transcoding: bool,
    pub enable_video_playback_transcoding: bool,
    pub enable_playback_remuxing: bool,
    pub force_remote_source_transcoding: bool,
    pub enable_content_deletion: bool,
    pub enable_content_deletion_from_folders: Option<Vec<String>>,
    pub enable_content_downloading: bool,
    pub enable_sync_transcoding: bool,
    pub enable_media_conversion: bool,
    pub enabled_devices: Option<Vec<String>>,
    pub enable_all_devices: bool,
    pub enabled_channels: Option<Vec<String>>,
    pub enable_all_channels: bool,
    pub enabled_folders: Option<Vec<String>>,
    pub enable_all_folders: bool,
    pub invalid_login_attempt_count: i32,
    pub login_attempts_before_lockout: i32,
    pub max_active_sessions: i32,
    pub enable_public_sharing: bool,
    pub blocked_media_folders: Option<Vec<String>>,
    pub blocked_channels: Option<Vec<String>>,
    pub remote_client_bitrate_limit: i32,
    pub authentication_provider_id: String,
    pub password_reset_provider_id: String,
    pub sync_play_access: SyncPlayUserAccessType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: Option<String>,
    pub server_id: Option<String>,
    pub server_name: Option<String>,
    pub id: String,
    pub primary_image_tag: Option<String>,
    pub enable_auto_login: Option<bool>,
    pub last_login_date: Option<String>,
    pub last_activity_date: Option<String>,
    pub configuration: Option<UserConfiguration>,
    pub policy: Option<UserPolicy>,
    pub primary_image_aspect_ratio: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub enum DlnaProfileType {
    Audio,
    Video,
    Photo,
    Subtitle,
    Lyric,
}

#[derive(Debug, Deserialize)]
pub enum TranscodeSeekInfo {
    Auto,
    Bytes,
}

#[derive(Debug, Deserialize)]
pub enum EncodingContext {
    Streaming,
    Static,
}

#[derive(Debug, Deserialize)]
pub enum CodecType {
    Video,
    VideoAudio,
    Audio,
}

#[derive(Debug, Deserialize)]
pub enum ProfileConditionType {
    Equals,
    NotEquals,
    LessThanEqual,
    GreaterThanEqual,
    EqualsAny,
}

#[derive(Debug, Deserialize)]
pub enum ProfileConditionValue {
    AudioChannels,
    AudioBitrate,
    AudioProfile,
    Width,
    Height,
    Has64BitOffsets,
    PacketLength,
    VideoBitDepth,
    VideoBitrate,
    VideoFramerate,
    VideoLevel,
    VideoProfile,
    VideoTimestamp,
    IsAnamorphic,
    RefFrames,
    NumAudioStreams,
    NumVideoStreams,
    IsSecondaryAudio,
    VideoCodecTag,
    #[serde(rename = "IsAvc")]
    IsAvc,
    IsInterlaced,
    AudioSampleRate,
    AudioBitDepth,
    VideoRangeType,
    NumStreams,
    VideoRotation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProfileCondition {
    pub condition: ProfileConditionType,
    pub property: ProfileConditionValue,
    pub value: Option<String>,
    pub is_required: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DirectPlayProfile {
    pub container: String,
    pub audio_codec: Option<String>,
    pub video_codec: Option<String>,
    #[serde(rename = "Type")]
    pub profile_type: DlnaProfileType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TranscodingProfile {
    pub container: String,
    #[serde(rename = "Type")]
    pub profile_type: DlnaProfileType,
    pub video_codec: String,
    pub audio_codec: String,
    pub protocol: MediaStreamProtocol,
    pub estimate_content_length: bool,
    pub enable_mpegts_m2ts_mode: bool,
    pub transcode_seek_info: TranscodeSeekInfo,
    pub copy_timestamps: bool,
    pub context: EncodingContext,
    pub enable_subtitles_in_manifest: bool,
    pub max_audio_channels: Option<String>,
    pub min_segments: i32,
    pub segment_length: i32,
    pub conditions: Vec<ProfileCondition>,
    pub enable_audio_vbr_encoding: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerProfile {
    #[serde(rename = "Type")]
    pub profile_type: DlnaProfileType,
    pub conditions: Vec<ProfileCondition>,
    pub container: Option<String>,
    pub sub_container: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodecProfile {
    #[serde(rename = "Type")]
    pub codec_type: CodecType,
    pub conditions: Vec<ProfileCondition>,
    pub apply_conditions: Vec<ProfileCondition>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub sub_container: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubtitleProfile {
    pub format: Option<String>,
    pub method: SubtitleDeliveryMethod,
    pub didl_mode: Option<String>,
    pub language: Option<String>,
    pub container: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceProfile {
    pub name: Option<String>,
    pub id: Option<String>,
    pub max_streaming_bitrate: Option<i32>,
    pub max_static_bitrate: Option<i32>,
    pub music_streaming_transcoding_bitrate: Option<i32>,
    pub max_static_music_bitrate: Option<i32>,
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    pub transcoding_profiles: Vec<TranscodingProfile>,
    pub container_profiles: Vec<ContainerProfile>,
    pub codec_profiles: Vec<CodecProfile>,
    pub subtitle_profiles: Vec<SubtitleProfile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientCapabilities {
    pub playable_media_types: Vec<MediaType>,
    pub supported_commands: Vec<GeneralCommandType>,
    pub supports_media_control: bool,
    pub supports_persistent_identifier: bool,
    pub device_profile: Option<DeviceProfile>,
    pub app_store_url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalUrl {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaUrl {
    pub url: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameGuidPair {
    pub name: Option<String>,
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserItemDataDto {
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
pub struct BaseItemPerson {
    pub name: Option<String>,
    pub id: String,
    pub role: Option<String>,
    #[serde(rename = "Type")]
    pub kind: PersonKind,
    pub primary_image_tag: Option<String>,
    pub image_blur_hashes: Option<ImageBlurHashes>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NowViewingItem {
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
    pub video_3d_format: Option<Video3DFormat>,
    pub premiere_date: Option<String>,
    pub external_urls: Option<Vec<ExternalUrl>>,
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
    pub remote_trailers: Option<Vec<MediaUrl>>,
    pub provider_ids: Option<HashMap<String, String>>,
    pub is_hd: Option<bool>,
    pub is_folder: Option<bool>,
    pub parent_id: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: Option<BaseItemKind>,
    pub people: Option<Vec<BaseItemPerson>>,
    pub studios: Option<Vec<NameGuidPair>>,
    pub genre_items: Option<Vec<NameGuidPair>>,
    pub parent_logo_item_id: Option<String>,
    pub parent_backdrop_item_id: Option<String>,
    pub parent_backdrop_image_tags: Option<Vec<String>>,
    pub local_trailer_count: Option<i32>,
    pub user_data: Option<UserItemDataDto>,
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
    pub album_primary_image_tag: Option<String>,
    pub series_primary_image_tag: Option<String>,
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
    pub parent_logo_image_tag: Option<String>,
    pub parent_art_item_id: Option<String>,
    pub parent_art_image_tag: Option<String>,
    pub series_thumb_image_tag: Option<String>,
    pub image_blur_hashes: Option<ImageBlurHashes>,
    pub series_studio: Option<String>,
    pub parent_thumb_item_id: Option<String>,
    pub parent_thumb_image_tag: Option<String>,
    pub parent_primary_image_item_id: Option<String>,
    pub parent_primary_image_tag: Option<String>,
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
    pub channel_primary_image_tag: Option<String>,
    pub start_date: Option<String>,
    pub completion_percentage: Option<f64>,
    pub is_repeat: Option<bool>,
    pub episode_title: Option<String>,
    pub channel_type: Option<ChannelType>,
    pub audio: Option<Audio>,
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
    pub current_program: Option<Box<NowViewingItem>>,
    pub original_language: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub play_state: Option<PlayerStateInfo>,
    pub additional_users: Option<Vec<SessionUserInfo>>,
    pub capabilities: Option<ClientCapabilities>,
    pub remote_end_point: Option<String>,
    pub playable_media_types: Vec<MediaType>,
    pub id: Option<String>,
    pub user_id: String,
    pub user_name: Option<String>,
    pub client: Option<String>,
    pub last_activity_date: String,
    pub last_playback_check_in: String,
    pub last_paused_date: Option<String>,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub now_playing_item: Option<NowViewingItem>,
    pub now_viewing_item: Option<NowViewingItem>,
    pub device_id: Option<String>,
    pub application_version: Option<String>,
    pub transcoding_info: Option<TranscodingInfo>,
    pub is_active: bool,
    pub supports_media_control: bool,
    pub supports_remote_control: bool,
    pub now_playing_queue: Option<Vec<QueueItem>>,
    pub has_custom_device_name: bool,
    pub playlist_item_id: Option<String>,
    pub server_id: Option<String>,
    pub user_primary_image_tag: Option<String>,
    pub supported_commands: Vec<GeneralCommandType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Authenticate {
    pub user: Option<User>,
    pub session_info: Option<SessionInfo>,
    pub access_token: Option<String>,
    pub server_id: Option<String>,
}

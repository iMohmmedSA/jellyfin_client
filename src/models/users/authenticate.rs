use serde::Deserialize;

use crate::models::items::{
    Item,
    enums::MediaType,
    media::{MediaStreamProtocol, SubtitleDeliveryMethod},
};

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
    pub now_playing_item: Option<Item>,
    pub now_viewing_item: Option<Item>,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Unknown,
    Video,
    Audio,
    Photo,
    Book,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind {
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
    #[serde(rename = "TV")]
    Tv,
    Radio,
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
pub enum LocationType {
    FileSystem,
    Remote,
    Virtual,
    Offline,
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
pub enum PlayAccess {
    Full,
    None,
}

//! Data models for songs.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#songs-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::{
    ArtistCoreNoMetadata, Referent, SongMetadata, Text, UserCore,
    UserInteractionMetadata,
};

/// A song response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SongResponse {
    /// Song data.
    pub song: Song,
}

/// Song data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Song {
    /// Apple Music ID.
    pub apple_music_id: Option<String>,
    /// Apple Music player URL.
    pub apple_music_player_url: String,
    /// Current user metadata.
    pub current_user_metadata: SongMetadata,
    /// Song description.
    pub description: Text,
    /// Embeddable content.
    pub embed_content: String,
    /// If the song has a featured video.
    pub featured_video: bool,
    /// The song's language.
    pub language: Option<String>,
    /// Reason for a lyrics placeholder.
    pub lyrics_placeholder_reason: Option<String>,
    /// Recording location.
    pub recording_location: Option<String>,
    /// Release date.
    pub release_date: Option<String>,
    /// Associated album.
    pub album: Option<Album>,
    /// Custom performances.
    pub custom_performances: Vec<Performance>,
    /// Song description as an annotation.
    pub description_annotation: Referent,
    /// User who has marked the song lyrics as complete.
    pub lyrics_marked_complete_by: Option<UserCore<UserInteractionMetadata>>,
    /// Staff who has marked the song lyrics as approved.
    pub lyrics_marked_staff_approved_by: Option<UserCore<UserInteractionMetadata>>,
    /// Associated media.
    pub media: Vec<Media>,
    /// Song producers.
    pub producer_artists: Vec<ArtistCoreNoMetadata>,
    /// Song relationships.
    pub song_relationships: Vec<SongRelationship>,
    /// Translation songs.
    pub translation_songs: Vec<TranslationSong>,
    /// Verified annotators.
    pub verified_annotations_by: Vec<UserCore<UserInteractionMetadata>>,
    /// Verified contributors.
    pub verified_contributors: Vec<Contribution>,
    /// Providers of verified lyrics.
    pub verified_lyrics_by: Vec<UserCore<UserInteractionMetadata>>,
    /// Song writers.
    pub writer_artists: Vec<ArtistCoreNoMetadata>,
    /// Core song data.
    #[serde(flatten)]
    pub core: SongCore<SongStats>,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Song media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Media {
    /// Song audio.
    Audio(Audio),
    /// Song video.
    Video(Video),
}

impl Default for Media {
    fn default() -> Self {
        Media::Audio(Audio::default())
    }
}

/// Song audio.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Audio {
    /// The user on the provider's platform providing the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// Native URI of the song in the provider's platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_uri: Option<String>,
    /// The audio provider platform.
    pub provider: String,
    /// URL to the audio.
    pub url: String,
}

/// Song video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Video {
    /// The video provider platform.
    pub provider: String,
    /// URL to the video.
    pub url: String,
    /// The start time of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
}

/// Song relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SongRelationship {
    /// Internal relationship type field.
    /// Included to stay consistent when serializing.
    #[serde(rename = "type")]
    rel_type: RelationshipType,
    /// The type of relationship.
    pub relationship_type: RelationshipType,
    /// URL to the page of relationships.
    pub url: Option<String>,
    /// Related songs.
    pub songs: Vec<SongCoreWithRDC<SongCoreStats>>,
}

/// A relationship between songs.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    /// Samples another song.
    Samples,
    /// Sampled in another song.
    SampledIn,
    /// Interpolates another song.
    Interpolates,
    /// Interpolated by another song.
    InterpolatedBy,
    /// Cover of another song.
    CoverOf,
    /// Covered by another song.
    CoveredBy,
    /// Remix of another song.
    RemixOf,
    /// Remixed by another song.
    RemixedBy,
    /// Live version of another song.
    LiveVersionOf,
    /// Performed live as another song.
    PerformedLiveAs,
    /// Translation of another song.
    TranslationOf,
    /// Translated by another song.
    Translations,
    /// Unknown relationship.
    #[default]
    Unknown,
}

/// Information about a contribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Contribution {
    /// Contributions made.
    pub contributions: Vec<String>,
    /// Artist profile associated with the contributor.
    pub artist: ArtistCoreNoMetadata,
    /// User profile associated with the contributor.
    pub user: Option<UserCore<UserInteractionMetadata>>,
}

/// Song statistics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct SongStats {
    /// Total number of accepted annotations.
    pub accepted_annotations: u32,
    /// Total number of contributors.
    pub contributors: u32,
    /// Total number of IQ earners.
    pub iq_earners: u32,
    /// Total number of transcribers.
    pub transcribers: u32,
    /// Total number of verified annotations.
    pub verified_annotations: u32,
    /// Core statistics.
    #[serde(flatten)]
    pub core: SongCoreStats,
}

/// Core song statistics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct SongCoreStats {
    /// Total number of concurrents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrents: Option<u16>,
    /// Total number of page views.
    #[serde(rename = "pageviews", skip_serializing_if = "Option::is_none")]
    pub page_views: Option<u32>,
    /// Total number of unreviewed annotations.
    pub unreviewed_annotations: u32,
    /// Whether the song is hot.
    pub hot: bool,
}

/// A custom performance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Performance {
    /// A descriptor of the performance.
    pub label: String,
    /// Artists associated with the performance.
    pub artists: Vec<ArtistCoreNoMetadata>,
}

/// An album.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Album {
    /// Genius API path to the album.
    pub api_path: String,
    /// URL for the cover art.
    pub cover_art_url: String,
    /// Full title.
    pub full_title: String,
    /// Genius ID.
    pub id: u32,
    /// Name of the album.
    pub name: String,
    /// A display-ready release date.
    pub release_date_for_display: Option<String>,
    /// Genius URL to the album.
    pub url: String,
    /// Album's artist.
    pub artist: ArtistCoreNoMetadata,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Song data with release date components.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SongCoreWithRDC<S> {
    /// The song's release date, split into components.
    pub release_date_components: Option<DateComponents>,
    /// Core song data.
    #[serde(flatten)]
    pub core: SongCore<S>,
}

/// Core song data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SongCore<S> {
    /// Total number of annotations.
    pub annotation_count: u32,
    /// Artist names.
    pub artist_names: String,
    /// Full title.
    pub full_title: String,
    /// URL to the header thumbnail image.
    pub header_image_thumbnail_url: String,
    /// URL to the header image.
    pub header_image_url: String,
    /// Genius ID of the lyrics owner.
    pub lyrics_owner_id: u32,
    /// Total number of pyongs.
    pub pyongs_count: Option<u32>,
    /// URL to the page of relationships.
    pub relationships_index_url: String,
    /// Display-ready release date.
    pub release_date_for_display: Option<String>,
    /// Display-ready release date, with abbreviated month.
    pub release_date_with_abbreviated_month_for_display: Option<String>,
    /// URL to the song art thumbnail image.
    pub song_art_image_thumbnail_url: String,
    /// URL to the song art image.
    pub song_art_image_url: String,
    /// Song stats.
    pub stats: S,
    /// Title with featured artists.
    pub title_with_featured: String,
    /// Featured artists.
    pub featured_artists: Vec<ArtistCoreNoMetadata>,
    /// The song's primary artist.
    pub primary_artist: ArtistCoreNoMetadata,
    /// Essential song data.
    #[serde(flatten)]
    pub essential: SongEssential,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A date by its components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DateComponents {
    /// The year.
    pub year: u16,
    /// The month.
    pub month: Option<u8>,
    /// The day.
    pub day: Option<u8>,
}

/// A translation song.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TranslationSong {
    /// The language of the translation song.
    pub language: Option<String>,
    /// Essential song data.
    #[serde(flatten)]
    pub essential: SongEssential,
}

/// Essential song data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SongEssential {
    /// Genius API path to the song.
    pub api_path: String,
    /// Genius ID.
    pub id: u32,
    /// State of the lyrics.
    pub lyrics_state: String,
    /// Genius path to the song.
    pub path: String,
    /// Song title.
    pub title: String,
    /// Genius URL to the song.
    pub url: String,
}

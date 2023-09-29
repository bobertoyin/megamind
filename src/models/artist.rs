//! Data models for artists.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#artists-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::{Referent, Text, UserCore, UserInteractionMetadata};

/// An artist response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtistResponse {
    /// Artist data.
    pub artist: Artist,
}

/// Artist data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Artist {
    /// Alternate names for the artist.
    pub alternate_names: Vec<String>,
    /// Text description of the artist.
    pub description: Text,
    /// Artist's Facebook name.
    pub facebook_name: Option<String>,
    /// Artist's Instagram name.
    pub instagram_name: Option<String>,
    /// Artist's Twitter name.
    pub twitter_name: Option<String>,
    /// Number of followers.
    pub followers_count: u32,
    /// Whether the artist is a translation artist.
    pub translation_artist: bool,
    /// Annotation associated with the artist description.
    pub description_annotation: Referent,
    /// User profile associated with the artist.
    pub user: Option<UserCore<UserInteractionMetadata>>,
    /// Core artist data.
    #[serde(flatten)]
    pub core: ArtistCore,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Core artist data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtistCore {
    /// Artist info with no metadata.
    #[serde(flatten)]
    pub info: ArtistCoreNoMetadata,
    /// Current user metadata.
    pub current_user_metadata: UserInteractionMetadata,
}

/// Core artist data, with no current user metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtistCoreNoMetadata {
    /// Genius API path to this artist.
    pub api_path: String,
    /// URL for a header image.
    pub header_image_url: String,
    /// Genius ID.
    pub id: u32,
    /// Genius IQ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iq: Option<u32>,
    /// Artist image URL.
    pub image_url: String,
    /// Whether the artist is meme verified.
    pub is_meme_verified: bool,
    /// Whether the artist is verified.
    pub is_verified: bool,
    /// The artist's name.
    pub name: String,
    /// URL to the artist's Genius page.
    pub url: String,
}

//! Data models for users.
//!
//! The endpoint for retreiving users isn't officially documented,
//! but you can visit the [Genius documentation](https://docs.genius.com/#account-h2)
//! for context on the response format via other endpoints.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::{ArtistCore, Text, UserMetadata};

/// A user response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserResponse {
    /// User data.
    pub user: User,
}

/// User data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    /// Personal description for the user.
    pub about_me: Text,
    /// URL for a custom header image.
    pub custom_header_image_url: Option<String>,
    /// Number of users followed.
    pub followed_users_count: u32,
    /// Number of followers.
    pub followers_count: u32,
    /// Display-friendly Genius IQ.
    pub iq_for_display: String,
    /// URL for user photo.
    pub photo_url: Option<String>,
    /// Display-friendly user roles.
    pub roles_for_display: Vec<Role>,
    /// User statistics.
    pub stats: Stats,
    /// User's artist profile.
    pub artist: Option<ArtistCore>,
    /// Core user data.
    #[serde(flatten)]
    pub core: UserCore<UserMetadata>,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Core user data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserCore<M> {
    /// Genius API path to this user.
    pub api_path: String,
    /// Avatar images for this user.
    pub avatar: Avatar,
    /// URL for a header image.
    pub header_image_url: String,
    /// Human-readable display-friendly user role.
    pub human_readable_role_for_display: Option<String>,
    /// Genius ID.
    pub id: u32,
    /// Genius IQ.
    pub iq: i32,
    /// User name (not sure why this is a dupe of [`UserCore::name`]).
    pub login: String,
    /// User name.
    pub name: String,
    /// Display-friendly user role.
    pub role_for_display: Option<Role>,
    /// Genius URL to the user.
    pub url: String,
    /// Current user metadata.
    pub current_user_metadata: M,
}

/// User roles.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]

pub enum Role {
    /// Genius staff.
    Regulator,
    /// Also Genius staff (it's hard to find an up-to-date list of these roles).
    Staff,
    /// Verified artist.
    VerifiedArtist,
    /// Editor who coaches contributers and resolves conflicts.
    Moderator,
    /// Contributor and content curator.
    Editor,
    /// Recognized leader in the forum community.
    Mediator,
    /// A normal contributor.
    Contributor,
    /// A transcriber.
    Transcriber,
}

/// Avatar images.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Avatar {
    /// Tiny image.
    pub tiny: Image,
    /// Thumbnail image.
    pub thumb: Image,
    /// Small image.
    pub small: Image,
    /// Medium image.
    pub medium: Image,
}

/// An image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    /// The URL to the image.
    pub url: String,
    /// The image's bounding box.
    pub bounding_box: BoundingBox,
}

/// A bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct BoundingBox {
    /// The width of the box in pixels.
    pub width: u16,
    /// The height of the box in pixels.
    pub height: u16,
}

/// User statistics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Stats {
    /// Number of annotations.
    pub annotations_count: u32,
    /// Number of answers.
    pub answers_count: u32,
    /// Number of comments.
    pub comments_count: u32,
    /// Number of forum posts.
    pub forum_posts_count: u32,
    /// Number of pyongs.
    pub pyongs_count: u32,
    /// Number of questions.
    pub questions_count: u32,
    /// Number of transcriptions.
    pub transcriptions_count: u32,
}

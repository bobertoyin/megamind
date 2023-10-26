//! Data models for annotations.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#annotations-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::{
    AnnotationMetadata, Metadata, ReferentCore, Role, Text, UserCore,
    UserInteractionMetadata, UserInteractions,
};

/// An annotation response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationResponse {
    /// An annotation.
    pub annotation: Annotation,
    /// The referent of the annotation.
    pub referent: ReferentCore,
}

/// Annotation data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Annotation {
    /// API path to the annotation.
    pub api_path: String,
    /// Content of the annotation.
    pub body: Text,
    /// Total number of comments on the annotation.
    pub comment_count: u32,
    /// Whether the annotation is a community post(?).
    pub community: bool,
    /// Custom preview element. Have yet to see any non-nulls in the wild.
    pub custom_preview: (),
    /// Whether the annotation has voters.
    pub has_voters: bool,
    /// Genius ID of the annotation.
    pub id: u32,
    /// Whether the annotation is pinned.
    pub pinned: bool,
    /// Sharable URL.
    pub share_url: String,
    /// Source element. Have yet to see any non-nulls in the wild.
    pub source: (),
    /// State of the annotation.
    pub state: AnnotationState,
    /// Genius URL to the annotation.
    pub url: String,
    /// Whether the annotation is verified.
    pub verified: bool,
    /// Total number of votes on the annotation.
    pub votes_total: i32,
    /// Rejection comment element.
    pub rejection_comment: Option<RejectionComment>,
    /// Cosigners of the annotation.
    pub cosigned_by: Vec<UserCore<Metadata<UserInteractions>>>,
    /// Verifier of the annotation.
    pub verified_by: Option<UserCore<Metadata<UserInteractions>>>,
    /// Authors of the annotation.
    pub authors: Vec<Attributions>,
    /// Current user metadata.
    pub current_user_metadata: AnnotationMetadata,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A rejection comment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RejectionComment {
    /// Genius API path.
    pub api_path: String,
    /// Text description.
    pub body: Text,
    /// Commentable ID.
    pub commentable_id: u32,
    /// Commentable type.
    pub commentable_type: String,
    /// Created at.
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Whether the comment has voters.
    pub has_voters: bool,
    /// Genius ID.
    pub id: u32,
    /// Pinned role.
    pub pinned_role: Option<String>,
    /// Total votes.
    pub votes_total: u32,
    /// Anonymous author.
    pub anonymous_author: (),
    /// Author.
    pub author: UserCore<UserInteractionMetadata>,
    /// Reason.
    pub reason: Option<RejectionReason>,
}

/// A rejection reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RejectionReason {
    /// Context URL.
    pub context_url: String,
    /// Display character.
    pub display_character: char,
    /// Handle.
    pub handle: String,
    /// Genius ID.
    pub id: u32,
    /// Name.
    pub name: String,
    /// Raw name.
    pub raw_name: String,
    /// Requires body.
    pub requires_body: bool,
    /// Slug.
    pub slug: String,
}

/// An attribution to an author.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Attributions {
    /// Portion of the annotation attributed to the user. Value is between 0 and 1.
    pub attribution: f64,
    /// The pinned role of the author.
    pub pinned_role: Option<Role>,
    /// The user profile of the author.
    pub user: UserCore<Metadata<UserInteractions>>,
}

/// The state of an annotation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationState {
    /// From a verified artist.
    #[default]
    Verified,
    /// Not fully annotated.
    NeedsExegesis,
    /// Approved for display.
    Accepted,
    /// Likely in need of improvement.
    Suspect,
    /// In need of review.
    Rough,
    /// Pending.
    Pending,
}

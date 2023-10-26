//! Data models for the various metadata fields in other models.
use serde::{Deserialize, Serialize};

/// Annotation metadata.
pub type AnnotationMetadata =
    MetadataWithActions<AnnotationActions, Metadata<AnnotationInteractions>>;
/// User metadata.
pub type UserMetadata = MetadataWithFeatures<Metadata<UserInteractions>>;
/// Metadata with user interactions.
pub type UserInteractionMetadata = Metadata<UserInteractions>;
/// Song metadata.
pub type SongMetadata = MetadataWithActions<
    SongActions,
    MetadataWithRelationships<SongRelationships, Metadata<SongInteractions>>,
>;

/// Metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Metadata<I> {
    /// Permissions.
    pub permissions: Vec<String>,
    /// Excluded permissions.
    pub excluded_permissions: Vec<String>,
    /// Interactions.
    pub interactions: I,
}

/// Metadata with actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataWithActions<A, M> {
    /// IQ actions.
    pub iq_by_action: A,
    /// Metadata.
    #[serde(flatten)]
    pub metadata: M,
}

/// Metadata with features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataWithFeatures<M> {
    /// Features.
    pub features: Vec<String>,
    /// Metadata.
    #[serde(flatten)]
    pub metadata: M,
}

/// Metadata with relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataWithRelationships<R, M> {
    /// Relationships.
    pub relationships: R,
    /// Metadata.
    #[serde(flatten)]
    pub metadata: M,
}

/// Possible interactions with an annotation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationInteractions {
    /// Whether the user cosigned an annotation.
    pub cosign: bool,
    /// Whether the user pyonged an annotation.
    pub pyong: bool,
    /// The vote on an annotation.
    pub vote: Option<Vote>,
}

/// User interactions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInteractions {
    /// If you are following the person.
    pub following: bool,
}

/// Song interactions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct SongInteractions {
    /// Whether the user followers a song.
    pub following: bool,
    /// Whether the user pyonged a song.
    pub pyong: bool,
}

/// Annotation actions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationActions {
    /// Accept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<PrimaryAction>,
    /// Reject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject: Option<PrimaryAction>,
    /// Delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<PrimaryAction>,
}

/// Song actions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct SongActions {
    /// Editing metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_metadata: Option<PrimaryAction>,
    /// Answering a question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_question: Option<PrimaryAction>,
}

/// Song relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SongRelationships {
    /// Pinned role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_role: Option<String>,
}

/// A primary action.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct PrimaryAction {
    /// The action.
    pub primary: Action,
}

/// An action
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Action {
    /// Whether the action is applicable.
    pub applicable: bool,
    /// The base value.
    pub base: f32,
    /// The multiplier value.
    pub multiplier: u32,
}

/// A vote.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Vote {
    /// Upvote.
    Up,
    /// Downvote.
    Down,
}

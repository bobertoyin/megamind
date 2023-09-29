//! Data models for referents.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#referents-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::annotation::Annotation;

/// A referents response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferentsResponse {
    /// A list of referents.
    pub referents: Vec<Referent>,
}

/// A referent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Referent {
    /// Annotations.
    pub annotations: Vec<Annotation>,
    /// Core referent data.
    #[serde(flatten)]
    pub core: ReferentCore,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Referent core data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferentCore {
    /// Referent type.
    #[serde(rename = "_type")]
    pub referent_type: ReferentType,
    /// Whether a referent is featured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    /// Genius ID of the annotator.
    pub annotator_id: u32,
    /// Genius name of the annotator.
    pub annotator_login: String,
    /// API path to the referent.
    pub api_path: String,
    /// Referent classification.
    pub classification: String,
    /// Referent fragment.
    pub fragment: String,
    /// Genius ID of the referent.
    pub id: u32,
    /// Whether the referent is a description.
    pub is_description: bool,
    /// Path to the referent.
    pub path: String,
    /// Referent range.
    pub range: Range,
    /// Song ID associated with the referent.
    pub song_id: Option<u32>,
    /// Genius URL to the referent.
    pub url: String,
    /// Genius IDs of verified annotators.
    pub verified_annotator_ids: Vec<u32>,
    /// The annotatable entity.
    pub annotatable: Annotatable,
}

/// A referent range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]

pub struct Range {
    /// Start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Start offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<String>,
    /// End.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// End offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<String>,
    /// Before content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// After content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Content.
    pub content: String,
}

/// An annotatable entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Annotatable {
    /// Genius API path.
    pub api_path: String,
    /// The entity's context.
    pub context: Option<String>,
    /// Genius ID.
    pub id: u32,
    /// Image URL.
    pub image_url: String,
    /// Link title.
    pub link_title: String,
    /// Title.
    pub title: String,
    /// Entity type.
    #[serde(rename = "type")]
    pub annotatable_type: String,
    /// Genius URL.
    pub url: String,
    /// Client interaction timestamps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_timestamps: Option<Timestamps>,
}

/// Client interaction timestamps.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Timestamps {
    /// The last time a human made an update.
    #[serde(with = "ts_seconds")]
    pub updated_by_human_at: DateTime<Utc>,
    /// The last time lyrics were updated.
    #[serde(with = "ts_seconds")]
    pub lyrics_updated_at: DateTime<Utc>,
}

/// Referent type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReferentType {
    /// A referent.
    #[default]
    Referent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referent_type_default() {
        assert_eq!(ReferentType::default(), ReferentType::Referent);
    }
}

//! Data models for search results.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#search-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::{SongCoreStats, SongCoreWithRDC};

/// A search response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResponse {
    /// Search hits.
    pub hits: Vec<Hit>,
}

/// A search hit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Hit {
    /// Song hit.
    Song(HitCore<SongCoreWithRDC<SongCoreStats>>),
}

/// Core search hit data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HitCore<R> {
    /// Details about search term matches in the hit. Seems to be empty for all hits.
    pub highlights: Vec<()>,
    /// Index of the hit.
    pub index: HitIndex,
    /// Resulting data.
    pub result: R,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Search hit index.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]

pub enum HitIndex {
    /// A song.
    #[default]
    Song,
}

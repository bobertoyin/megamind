//! Data models for web pages.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#web_pages-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

/// A web page response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPageResponse {
    /// Web page data.
    pub web_page: WebPage,
}

/// Web page data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WebPage {
    /// API path to the web page.
    pub api_path: Option<String>,
    /// Domain of the web page.
    pub domain: String,
    /// ID of the web page.
    pub id: Option<u32>,
    /// Normalized URL.
    pub normalized_url: String,
    /// Full URL.
    pub share_url: String,
    /// Page title.
    pub title: String,
    /// Full URL.
    pub url: String,
    /// Total number of annotations on this page.
    pub annotation_count: u32,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

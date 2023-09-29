//! Data models for the API endpoints.
use serde::{Deserialize, Serialize};

pub mod account;
pub use account::*;
pub mod annotation;
pub use annotation::*;
pub mod artist;
pub use artist::*;
pub mod metadata;
pub use metadata::*;
pub mod referent;
pub use referent::*;
pub mod search;
pub use search::*;
pub mod song;
pub use song::*;
pub mod user;
pub use user::*;
pub mod webpage;
pub use webpage::*;

/// An endpoint response.
///
/// Necessary because the Genius API wraps the response payload with some metadata about the response.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Response<T> {
    /// Means the request was successful.
    Success {
        /// The response metadata.
        meta: SuccessMeta,
        /// The response payload.
        response: T,
    },
    /// Means an error occured during the request.
    Error {
        /// The response metadata.
        meta: ErrorMeta,
        /// An optional error response payload.
        #[serde(skip_serializing_if = "Option::is_none")]
        response: Option<ErrorResponse>,
    },
    /// Means that the request failed due to some reason outside the typical error cases.
    /// This [isn't officially documented as a response format](https://docs.genius.com/#/response-format-h1),
    /// so it's a bit of a catchall for when a response doesn't match [`Response::Error`].
    Other {
        /// The name of the error.
        error: String,
        /// The description of the error.
        error_description: String,
    },
}

/// An error response payload.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct ErrorResponse {
    /// The error message.
    pub error: String,
}

/// Metadata for successful responses to requests.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Default)]
pub struct SuccessMeta {
    /// The HTTP status code.
    pub status: u16,
}

/// Metadata for error responses to requests.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct ErrorMeta {
    /// The HTTP status code.
    pub status: u16,
    /// The error message.
    pub message: String,
}

/// Textual content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Text {
    /// Plain text.
    pub plain: String,
    /// Unescaped HTML text.
    pub html: String,
}

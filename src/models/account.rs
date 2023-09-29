//! Data models for accounts.
//!
//! Visit the [Genius documentation](https://docs.genius.com/#account-h2) for more context.
#[cfg(feature = "catchall")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "catchall")]
use serde_json::Value;

use super::User;

/// An account response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResponse {
    /// User account data.
    #[serde(rename = "user")]
    pub account: Account,
}

/// Account data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Account {
    /// Email address.
    pub email: String,
    /// Number of unread items in groups inbox.
    pub unread_groups_inbox_count: u32,
    /// Number of unread items in main activity inbox.
    pub unread_main_activity_inbox_count: u32,
    /// Number of unread items in newsfeed inbox.
    pub unread_newsfeed_inbox_count: u32,
    /// Number of unread messages.
    pub unread_messages_count: u32,
    /// Core user account data.
    #[serde(flatten)]
    pub user: User,
    /// Extra data.
    #[cfg(feature = "catchall")]
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

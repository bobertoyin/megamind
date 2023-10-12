#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use log::info;
use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue, AUTHORIZATION},
    Client as ReqwestClient, Error as ReqwestError,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_slice, Error as JsonError};
use thiserror::Error;

pub mod models;
use models::*;

/// The base URL for the API.
pub const BASE_URL: &str = "https://api.genius.com";

/// Client errors.
#[derive(Debug, Error)]
pub enum ClientError {
    /// An error related to the act of sending and receiving over HTTP.
    #[error("HTTP request error: {0}")]
    HttpError(#[from] ReqwestError),
    /// An error related to parsing an HTTP response body as JSON.
    #[error("JSON parse error: {0}")]
    JsonError(#[from] JsonError),
}

/// An HTTP client for interacting with the Genius API.
///
/// Essentially just a thin wrapper around [`reqwest::Client`],
/// meaning that if you want more control/need to access a missing endpoint
/// then you can just use the data models with Reqwest directly.
///
/// This also means that you can clone this client freely
/// and **should not** use [`std::sync::Arc`] or [`std::rc::Rc`], much like [`reqwest::Client`].
#[derive(Debug, Clone)]
pub struct Client {
    // internal Reqwest client
    internal: ReqwestClient,
}

impl Client {
    /// Make a generic GET request at a specified relative endpoint.
    ///
    /// # Args
    ///
    /// * `endpoint` - The relative endpoint; should have "/" prepended.
    /// * `query` - Any query parameters; matches the signature for [`reqwest::RequestBuilder::query`].
    ///
    /// # Returns
    ///
    /// A [`Response`].
    /// [`reqwest::Error`]s can occur if the request fails at the [`reqwest`] level, which includes HTTP related things and JSON parsing.
    async fn get<T: DeserializeOwned, S: AsRef<str>, P: Serialize + AsRef<str>>(
        &self,
        endpoint: S,
        query: &[(&str, P)],
    ) -> Result<Response<T>, ClientError> {
        info!(
            target: "megamind::get",
            "endpoint: \"{}\", queries: \"{}\"",
            endpoint.as_ref(),
            query
                .iter()
                .map(|q| format!("{}={}", q.0, q.1.as_ref()))
                .collect::<Vec<String>>()
                .join(",")
        );
        let text = self
            .internal
            .get(format!("{}{}", BASE_URL, endpoint.as_ref()))
            .query(query)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(from_slice(&text)?)
    }

    /// Get the account info for the currently authed user.
    ///
    /// Requires scope: `me`.
    ///
    /// # Returns
    ///
    /// The current user.
    pub async fn account(&self) -> Result<Response<AccountResponse>, ClientError> {
        self.get("/account", &[("text_format", "html,plain")]).await
    }

    ///  Get an annotation.
    ///
    /// # Args
    ///
    /// * `id` - A Genius ID.
    ///
    /// # Returns
    ///
    /// The annotation associated with the ID.
    pub async fn annotation(
        &self,
        id: u32,
    ) -> Result<Response<AnnotationResponse>, ClientError> {
        self.get(
            format!("/annotations/{}", id),
            &[("text_format", "html,plain")],
        )
        .await
    }

    /// Get an artist.
    ///
    /// # Args
    ///
    /// * `id` - A Genius ID.
    ///
    /// # Returns
    ///
    /// The artist associated with the ID.
    pub async fn artist(
        &self,
        id: u32,
    ) -> Result<Response<ArtistResponse>, ClientError> {
        self.get(format!("/artists/{}", id), &[("text_format", "html,plain")])
            .await
    }

    /// Get referents.
    ///
    /// # Args
    ///
    /// * `created_by` - A Genius ID.
    /// * `associated` - The associated web page or song.
    /// * `per_page` - A per-page limit.
    /// * `page` - A page offset, starting at 1.
    ///
    /// # Returns
    ///
    /// The referents that are associated with the web page or song
    /// and/or are created by a user with the given Genius ID.
    /// Results follow the `per_page` and `page` rules, and there are
    /// some failure cases that the argument types can't prevent so please
    /// visit the [Genius documentation](https://docs.genius.com/#referents-h2) for more information.
    pub async fn referents(
        &self,
        created_by: Option<u32>,
        associated: Option<ReferentAssociation>,
        per_page: Option<u8>,
        page: Option<u8>,
    ) -> Result<Response<ReferentsResponse>, ClientError> {
        let mut queries = vec![("text_format", String::from("html,plain"))];
        if let Some(created_by_id) = created_by {
            queries.push(("created_by_id", created_by_id.to_string()));
        }
        if let Some(association) = associated {
            let params = match association {
                ReferentAssociation::SongId(id) => ("song_id", id.to_string()),
                ReferentAssociation::WebPageId(id) => ("web_page_id", id.to_string()),
            };
            queries.push(params);
        }
        if let Some(per_page) = per_page {
            queries.push(("per_page", per_page.to_string()));
        }
        if let Some(page) = page {
            queries.push(("page", page.to_string()));
        }
        self.get("/referents", &queries).await
    }

    /// Get search results.
    ///
    /// # Args
    ///
    /// * `query` - A search term to match against.
    ///
    /// # Returns
    ///
    /// Search results associated with the query.
    pub async fn search(
        &self,
        query: &str,
    ) -> Result<Response<SearchResponse>, ClientError> {
        self.get("/search", &[("q", query)]).await
    }

    /// Get a song.
    ///
    /// # Args
    ///
    /// * `id` - A Genius ID.
    ///
    /// # Returns
    ///
    /// The song associated with the ID.
    pub async fn song(&self, id: u32) -> Result<Response<SongResponse>, ClientError> {
        self.get(format!("/songs/{}", id), &[("text_format", "html,plain")])
            .await
    }

    /// Get a user.
    ///
    /// # Args
    ///
    /// * `id` - A Genius ID.
    ///
    /// # Returns
    ///
    /// The user associated with the ID.
    pub async fn user(&self, id: u32) -> Result<Response<UserResponse>, ClientError> {
        self.get(format!("/users/{}", id), &[("text_format", "html,plain")])
            .await
    }

    /// Get a web page.
    ///
    /// # Args
    ///
    /// * `raw_annotatable_url` - The URL as it would appear in a browser.
    /// * `canonical_url` - The URL as specified by an appropriate <link> tag in a page's <head>.
    /// * `og_url` - The URL as specified by an og:url <meta> tag in a page's <head>.
    ///
    /// # Returns
    ///
    /// The web page associated with the above arguments.
    pub async fn web_pages(
        &self,
        raw_annotatable_url: Option<&str>,
        canonical_url: Option<&str>,
        og_url: Option<&str>,
    ) -> Result<Response<WebPageResponse>, ClientError> {
        let mut queries = Vec::new();
        if let Some(rau) = raw_annotatable_url {
            queries.push(("raw_annotatable_url", rau));
        }
        if let Some(cu) = canonical_url {
            queries.push(("canonical_url", cu));
        }
        if let Some(ou) = og_url {
            queries.push(("og_url", ou));
        }
        self.get("/web_pages/lookup", &queries).await
    }
}

/// A web page or song ID that is associated with a referent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferentAssociation {
    /// A song via Genius ID.
    SongId(u32),
    /// A web page via Genius ID.
    WebPageId(u32),
}

/// Builder for [`Client`]s.
#[derive(Default, Debug, Clone)]
pub struct ClientBuilder {
    /// auth token
    auth_token: Option<String>,
}

impl ClientBuilder {
    /// Create a new [`ClientBuilder`].
    ///
    /// # Returns
    ///
    /// A new [`ClientBuilder`], with the base API URL configured to the production API URL.
    pub fn new() -> Self {
        ClientBuilder { auth_token: None }
    }

    /// Set the auth token.
    ///
    /// **Note**: does not protect you from entering invalid tokens (e.g., an empty string, an expired token, token with invalid characters, etc.).
    ///
    /// # Args
    ///
    /// * `auth_token` - The auth token for API requests.
    ///
    /// # Returns
    ///
    /// The modified [`ClientBuilder`].
    pub fn auth_token<S: Into<String>>(mut self, auth_token: S) -> Self {
        self.auth_token = Some(auth_token.into());
        self
    }

    /// Build a [`Client`].
    ///
    /// # Returns
    /// A configured [`Client`].
    /// [`ClientBuilderError`]s can occur if the auth token is missing or contains invalid characters.
    /// [`ClientBuilderError::ReqwestBuilder`] can technically happen but it wouldn't be clear as to why it would occur.
    pub fn build(self) -> Result<Client, ClientBuilderError> {
        if let Some(auth_token) = self.auth_token {
            let mut headers = HeaderMap::new();
            let mut header_val =
                HeaderValue::from_str(&format!("Bearer {}", auth_token))?;
            header_val.set_sensitive(true);
            headers.insert(AUTHORIZATION, header_val);
            Ok(Client {
                internal: ReqwestClient::builder().default_headers(headers).build()?,
            })
        } else {
            Err(ClientBuilderError::MissingAuthToken)
        }
    }
}

/// Errors that can occur during [`ClientBuilder::build`].
#[derive(Debug, Error)]
pub enum ClientBuilderError {
    /// Missing auth token.
    #[error("missing auth token")]
    MissingAuthToken,
    /// [`reqwest::ClientBuilder::build`] failed.
    #[error("internal client build error: {0}")]
    ReqwestBuilder(#[from] ReqwestError),
    /// Invalid value for the [`reqwest::header::AUTHORIZATION`] header.
    #[error("invalid auth header value: {0}")]
    AuthHeaderValue(#[from] InvalidHeaderValue),
}

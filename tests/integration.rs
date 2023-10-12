use std::env::var;

use assert_json_diff::{assert_json_eq, assert_json_include};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client as ReqwestClient,
};
use rstest::*;
use serde_json::{from_slice, to_value, Value};
use tokio::join;

use megamind::{Client, ClientBuilder, ReferentAssociation};

#[fixture]
fn token() -> String {
    let _ = env_logger::builder().is_test(true).try_init();
    var("GENIUS_TOKEN").unwrap()
}

#[fixture]
pub fn client(token: String) -> Client {
    ClientBuilder::new().auth_token(token).build().unwrap()
}

#[fixture]
pub fn reqwest_client(token: String) -> ReqwestClient {
    let mut headers = HeaderMap::new();
    let mut header_val = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
    header_val.set_sensitive(true);
    headers.insert(AUTHORIZATION, header_val);
    ReqwestClient::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

#[rstest]
#[tokio::test]

async fn test_account(client: Client, reqwest_client: ReqwestClient) {
    let (result, expected) = join!(
        client.account(),
        reqwest_client
            .get("https://api.genius.com/account?text_format=plain,html")
            .send()
            .await
            .unwrap()
            .bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

#[rstest]
#[tokio::test]

async fn test_annotation(
    #[values(16292, 34112, 10225840, 999999999)] id: u32,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let (result, expected) = join!(
        client.annotation(id),
        reqwest_client
            .get(format!(
                "https://api.genius.com/annotations/{}?text_format=plain,html",
                id
            ))
            .send()
            .await
            .unwrap()
            .bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

#[rstest]
#[tokio::test]

async fn test_artist(
    #[values(1234, 1421, 13440, 16775, 999999, 999999999)] id: u32,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let (result, expected) = join!(
        client.artist(id),
        reqwest_client
            .get(format!(
                "https://api.genius.com/artists/{}?text_format=plain,html",
                id
            ))
            .send()
            .await
            .unwrap()
            .bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

#[rstest]
#[tokio::test]

async fn test_referents(
    #[values(None, Some(12), Some(999999999))] created_by: Option<u32>,
    #[values(
        None,
        Some(ReferentAssociation::SongId(13440)),
        Some(ReferentAssociation::SongId(999999999)),
        Some(ReferentAssociation::WebPageId(10347))
    )]
    associated: Option<ReferentAssociation>,
    #[values(None, Some(0), Some(5))] per_page: Option<u8>,
    #[values(None, Some(0), Some(5))] page: Option<u8>,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let mut url = String::from("https://api.genius.com/referents/?");
    if let Some(created_by_id) = created_by {
        url.push_str(&format!("created_by_id={}&", created_by_id));
    }
    if let Some(association) = associated {
        let string = match association {
            ReferentAssociation::SongId(id) => format!("song_id={}&", id),
            ReferentAssociation::WebPageId(id) => format!("web_page_id={}&", id),
        };
        url.push_str(&string);
    }
    if let Some(pp) = per_page {
        url.push_str(&format!("per_page={}&", pp));
    }
    if let Some(p) = page {
        url.push_str(&format!("page={}&", p));
    }
    url.push_str("text_format=plain,html");

    let (result, expected) = join!(
        client.referents(created_by, associated, per_page, page),
        reqwest_client.get(url).send().await.unwrap().bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

#[rstest]
#[tokio::test]

async fn test_search(
    #[values("", "Kendrick Lamar", "Drake", "Mask Of", "Ella Fitzgerald", "Mozart")]
    query: &str,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let (result, expected) = join!(
        client.search(query),
        reqwest_client
            .get(format!("https://api.genius.com/search?q={}", query))
            .send()
            .await
            .unwrap()
            .bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

#[rstest]
#[tokio::test]

async fn test_song(
    #[values(8145634, 2177076, 7756301, 6691103, 7327436, 5444192, 99999, 99999999)] id: u32,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let (result, expected) = join!(
        client.song(id),
        reqwest_client
            .get(format!(
                "https://api.genius.com/songs/{}?text_format=plain,html",
                id
            ))
            .send()
            .await
            .unwrap()
            .bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_include!(actual: result_json, expected: expected_json);
}

#[rstest]
#[tokio::test]

async fn test_user(
    #[values(1234, 1421, 13440, 16775, 999999, 999999999)] id: u32,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let (result, expected) = join!(
        client.user(id),
        reqwest_client
            .get(format!(
                "https://api.genius.com/users/{}?text_format=plain,html",
                id
            ))
            .send()
            .await
            .unwrap()
            .bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

#[rstest]
#[tokio::test]

async fn test_web_pages(
    #[values(None, Some("https://docs.genius.com"), Some("https://foobar.com"))]
    raw_annotatable_url: Option<&str>,
    #[values(None, Some("https://docs.genius.com"), Some("https://foobar.com"))]
    canonical_url: Option<&str>,
    #[values(None, Some("https://docs.genius.com"), Some("https://foobar.com"))] og_url: Option<&str>,
    client: Client,
    reqwest_client: ReqwestClient,
) {
    let mut url = String::from("https://api.genius.com/web_pages/lookup?");
    if let Some(rau) = raw_annotatable_url {
        url.push_str(&format!("raw_annotatable_url={}&", rau));
    }
    if let Some(cu) = canonical_url {
        url.push_str(&format!("canonical_url={}&", cu));
    }
    if let Some(ou) = og_url {
        url.push_str(&format!("og_url={}&", ou));
    }

    let (result, expected) = join!(
        client.web_pages(raw_annotatable_url, canonical_url, og_url),
        reqwest_client.get(&url).send().await.unwrap().bytes()
    );

    let result_json = to_value(result.unwrap()).unwrap();
    let expected_json = from_slice::<Value>(&expected.unwrap()).unwrap();
    assert_json_eq!(result_json, expected_json);
}

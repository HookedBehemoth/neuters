use serde::Serialize;

use super::{common::Articles, error::ApiError, fetch::fetch};

pub fn fetch_articles_by_section(path: &str, size: u32) -> Result<Articles, ApiError> {
    #[derive(Serialize)]
    struct SectionQuery<'a> {
        size: u32,
        section_id: &'a str,
        website: &'static str,
    }
    let query = SectionQuery {
        size,
        section_id: path,
        website: "reuters",
    };

    let query_json = serde_json::to_string(&query).unwrap();

    const API_URL: &str =
        "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-section-alias-or-id-v1";
    fetch(API_URL, &query_json)
}

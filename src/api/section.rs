use super::{common::Articles, error::ApiError, fetch::fetch};

const API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-section-alias-or-id-v1";

pub fn fetch_articles_by_section(path: &str, size: u32) -> Result<Articles, ApiError> {
    let query = format!(
        r#"{{"size":{},"section_id":"{}","website":"reuters"}}"#,
        size, path
    );

    fetch(API_URL, &query)
}

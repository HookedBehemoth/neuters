use super::{common::Articles, error::ApiResult, fetch::fetch};

const API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-section-alias-or-id-v1";

pub fn fetch_articles_by_section(
    client: &ureq::Agent,
    path: &str,
    size: u32,
) -> ApiResult<Articles> {
    let query = format!(r#"{{"size":{size},"section_id":"{path}","website":"reuters"}}"#);

    fetch(client, API_URL, &query)
}

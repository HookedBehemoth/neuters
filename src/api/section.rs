use super::{common::Articles, error::ApiResult, fetch::fetch};

const API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/recent-stories-by-sections-v1";

pub fn fetch_articles_by_section(
    client: &ureq::Agent,
    path: &str,
    offset: u32,
    size: u32,
) -> ApiResult<Articles> {
    let query = format!(
        r#"{{"offset":{offset},"size":{size},"section_ids":"{path}","website":"reuters"}}"#
    );

    fetch(client, API_URL, &query)
}

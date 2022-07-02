use super::{common::Articles, error::ApiResult, fetch::fetch};

const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-search-v2";

pub fn fetch_articles_by_search(
    client: &ureq::Agent,
    keyword: &str,
    offset: u32,
    size: u32,
) -> ApiResult<Articles> {
    let query = format!(
        r#"{{"keyword":"{keyword}","offset":{offset},"orderby":"display_date:desc","size":{size},"website":"reuters"}}"#
    );

    fetch(client, API_URL, &query)
}

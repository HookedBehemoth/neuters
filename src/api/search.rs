use super::{common::Articles, error::ApiError, fetch::fetch};

const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-search-v2";

pub fn fetch_articles_by_search(
    keyword: &str,
    offset: u32,
    size: u32,
) -> Result<Articles, ApiError> {
    let query = format!(
        r#"{{"keyword":"{}","offset":{},"orderby":"display_date:desc","size":{},"website":"reuters"}}"#,
        keyword, offset, size
    );

    fetch(API_URL, &query)
}

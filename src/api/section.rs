use crate::client::Client;

use super::{common::{Articles, Section}, error::ApiResult, fetch::fetch};

const API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/recent-stories-by-sections-v1";

pub fn fetch_articles_by_section(
    client: &Client,
    path: &str,
    offset: u32,
    size: u32,
) -> ApiResult<Articles> {
    let query = format!(
        r#"{{"offset":{offset},"size":{size},"section_ids":"{path}","website":"reuters"}}"#
    );

    fetch(client, API_URL, &query)
}

const SITE_HIERARCHY_API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/site-hierarchy-by-name-v1";

pub fn fetch_site_hierarchy_by_name(
    client: &Client,
) -> ApiResult<Section> {

    fetch(client, SITE_HIERARCHY_API_URL, "")
}
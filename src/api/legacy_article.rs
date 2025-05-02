use serde::Deserialize;

use crate::{api::error::ApiError, client::Client};

use super::error::ApiResult;

#[derive(Deserialize)]
pub struct LegacyArticle {
    pub props: LegacyArticleProps,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyArticleProps {
    pub initial_state: LegacyArticleInitialState,
}

#[derive(Deserialize)]
pub struct LegacyArticleInitialState {
    pub article: LegacyArticleArticle,
}

#[derive(Deserialize)]
pub struct LegacyArticleArticle {
    pub stream: Box<[LegacyArticleStream]>,
}

#[derive(Deserialize)]
pub struct LegacyArticleStream {
    pub headline: String,
    pub description: String,
    pub date: LegacyArticleDate,
    pub authors: Box<[LegacyArticleAuthor]>,
    pub body_items: Box<[LegacyArticleBodyItem]>,
}

#[derive(Deserialize)]
pub struct LegacyArticleDate {
    pub published: String,
}

#[derive(Deserialize)]
pub struct LegacyArticleAuthor {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct LegacyArticleBodyItem {
    pub r#type: String,
    pub content: String,
}

pub fn fetch_legacy_article(client: &Client, path: &str) -> Result<ureq::Response, ureq::Error> {
    let link = format!("https://www.reuters.com{path}");

    client.get(&link).call()
}

pub fn parse_legacy_article(request: ureq::Response) -> ApiResult<LegacyArticle> {
    let html = request.into_string()?;

    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .get_element_by_id("__NEXT_DATA__")
        .ok_or_else(|| ApiError::Internal("Failed to parse Internet News article".to_owned()))?
        .get(parser)
        .ok_or_else(|| ApiError::Internal("Failed to parse Internet News article".to_owned()))?;
    let json = element.inner_text(parser);

    let json = serde_json::from_str(&json)?;

    Ok(json)
}

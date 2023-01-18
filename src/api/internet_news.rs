use std::time::Instant;

use serde::Deserialize;

use crate::api::error::ApiError;

use super::error::ApiResult;

#[derive(Deserialize)]
pub struct InternetNews {
    pub props: InternetNewsProps,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternetNewsProps {
    pub initial_state: InternetNewsInitialState,
}

#[derive(Deserialize)]
pub struct InternetNewsInitialState {
    pub article: InternetNewsArticle,
}

#[derive(Deserialize)]
pub struct InternetNewsArticle {
    pub stream: Vec<InternetNewsStream>,
}

#[derive(Deserialize)]
pub struct InternetNewsStream {
    pub id: String,
    pub headline: String,
    pub description: String,
    pub date: InternetNewsDate,
    pub authors: Vec<InternetNewsAuthor>,
    pub body_items: Vec<InternetNewsBodyItem>,
}

#[derive(Deserialize)]
pub struct InternetNewsDate {
    pub published: String,
}

#[derive(Deserialize)]
pub struct InternetNewsAuthor {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct InternetNewsBodyItem {
    pub r#type: String,
    pub content: String,
}

pub fn fetch_internet_news(client: &ureq::Agent, path: &str) -> ApiResult<InternetNews> {
    let link = format!("https://www.reuters.com{path}");
    println!("-> {link}");
    let request = client.get(&link).call()?;
    let html = request.into_string()?;

    let start = Instant::now();
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .get_element_by_id("__NEXT_DATA__")
        .ok_or_else(|| ApiError::Internal("Failed to parse Internet News article".to_owned()))?
        .get(parser)
        .ok_or_else(|| ApiError::Internal("Failed to parse Internet News article".to_owned()))?;
    let json = element.inner_text(parser);
    let elapsed = Instant::now() - start;
    println!("Elapsed: {}ms", elapsed.as_secs_f64() * 1000.0);

    let json = serde_json::from_str(&json)?;

    Ok(json)
}

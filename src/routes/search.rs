use crate::api::{
    common::Articles, error::ApiResult, search::fetch_articles_by_search,
    section::fetch_articles_by_section, topic::fetch_articles_by_topic,
};
use crate::document;
use maud::html;

pub fn render_topic(
    client: &ureq::Agent,
    path: &str,
    offset: u32,
    size: u32,
) -> ApiResult<String> {
    render_articles(fetch_articles_by_topic(client, path, offset, size)?)
}

pub fn render_section(client: &ureq::Agent, path: String, size: u32) -> ApiResult<String> {
    render_articles(fetch_articles_by_section(client, &path, size)?)
}

pub fn render_search(
    client: &ureq::Agent,
    keywords: &str,
    offset: u32,
    size: u32,
) -> ApiResult<String> {
    render_articles(fetch_articles_by_search(client, keywords, offset, size)?)
}

fn render_articles(articles: Articles) -> ApiResult<String> {
    let doc = document!(
        "Neuters - Reuters Proxy",
        html! {
            ul {
                @for article in articles.articles {
                    li { a href=(&article.canonical_url) { (&article.title) } }
                }
            }
        },
    );

    Ok(doc.into_string())
}

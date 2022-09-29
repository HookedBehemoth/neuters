use crate::api::{
    common::Articles, error::ApiResult, search::fetch_articles_by_search,
    section::fetch_articles_by_section, topic::fetch_articles_by_topic,
};
use crate::document;
use maud::html;

enum SearchType {
    Topic,
    Section,
}

pub fn render_topic(client: &ureq::Agent, path: &str, offset: u32, size: u32) -> ApiResult<String> {
    let article = fetch_articles_by_topic(client, path, offset, size)?;
    render_articles(article, path, offset, size, SearchType::Topic)
}

pub fn render_section(
    client: &ureq::Agent,
    path: &str,
    offset: u32,
    size: u32,
) -> ApiResult<String> {
    let article = fetch_articles_by_section(client, path, offset, size)?;
    render_articles(article, path, offset, size, SearchType::Section)
}

pub fn render_search(client: &ureq::Agent, request: &rouille::Request) -> ApiResult<String> {
    match request.get_param("query") {
        Some(query) => {
            let offset = request
                .get_param("offset")
                .map_or(0, |s| s.parse::<u32>().unwrap_or(0));
            let size = request
                .get_param("size")
                .map_or(20, |s| s.parse::<u32>().unwrap_or(20))
                .clamp(1, 20);

            render_search_impl(
                Some(fetch_articles_by_search(client, &query, offset, size)?),
                &query,
            )
        }
        _ => render_search_impl(None, ""),
    }
}

fn render_search_impl(articles: Option<Articles>, query: &str) -> ApiResult<String> {
    let doc = document!(
        "Neuters - Reuters Proxy - Search",
        html! {
            h1 { "Search: " (query) }
            form {
                input type="text" name="query" placeholder="Keywords..." value=(query) required="";
                button type="submit" { "Search" }
            }
            @if let Some(articles) = articles {
                ul {
                    @for article in articles.articles {
                        li { a href=(&article.canonical_url) { (&article.title) } }
                    }
                }
            }
        },
    );

    Ok(doc.into_string())
}

fn render_articles(
    articles: Articles,
    path: &str,
    offset: u32,
    steps: u32,
    search_type: SearchType,
) -> ApiResult<String> {
    let title = match search_type {
        SearchType::Section => articles.section.as_ref().map(|s| s.name.as_str()),
        SearchType::Topic => articles
            .topics
            .as_ref()
            .map(|t| t.get(0).map(|t| t.name.as_str()))
            .flatten(),
    }
    .unwrap_or("");

    let count = articles.articles.len() as u32;
    let total = articles.pagination.total_size;
    let (has_prev, has_next) = (offset > 0, offset + count < total);
    let prev_page = if has_prev {
        Some(format!("{path}?offset={}", offset.saturating_sub(count)))
    } else {
        None
    };
    let next_page = if has_next {
        Some(format!(
            "{path}?offset={}",
            offset.saturating_add(steps).min(total - 1)
        ))
    } else {
        None
    };

    let doc = document!(
        "Neuters - Reuters Proxy",
        html! {
            h1 { (title) }
            ul {
                @for article in articles.articles {
                    li { a href=(&article.canonical_url) { (&article.title) } }
                }
            }
            div.nav {
                a href=[prev_page] { "<" }
                ((offset + 1)) " to " ((offset + count)) " of " (total)
                a href=[next_page] { ">" }
            }
        },
    );

    Ok(doc.into_string())
}

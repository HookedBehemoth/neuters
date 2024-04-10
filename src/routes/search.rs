use crate::api::{
    common::Articles, error::ApiResult, search::fetch_articles_by_search,
    section::fetch_articles_by_section, topic::fetch_articles_by_topic,
};
use crate::document;
use maud::html;

#[derive(PartialEq)]
enum SearchType {
    Topic,
    Section,
    Query,
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

            let articles = fetch_articles_by_search(client, &query, offset, size)?;

            render_articles(articles, &query, offset, size, SearchType::Query)
        }
        _ => {
            let doc = document!(
                "Neuters - Reuters Proxy - Search",
                html! {
                    h1 { "Search:" }
                    form {
                        input type="text" name="query" placeholder="Keywords..." required="";
                        button type="submit" { "Search" }
                    }
                },
            );

            Ok(doc.into_string())
        }
    }
}

fn render_articles(
    articles: Articles,
    path: &str,
    offset: u32,
    steps: u32,
    search_type: SearchType,
) -> ApiResult<String> {
    let (title, url) = match search_type {
        SearchType::Section => (
            articles
                .section
                .as_ref()
                .map(|s| s.name.as_str())
                .unwrap_or(""),
            format!("{path}?"),
        ),
        SearchType::Topic => (
            articles
                .topics
                .as_ref()
                .and_then(|t| t.get(0).map(|t| t.name.as_str()))
                .unwrap_or(""),
            format!("{path}?"),
        ),
        SearchType::Query => (path, format!("/search?query={path}&")),
    };

    let count = articles
        .articles
        .as_ref()
        .map(|a| a.len() as u32)
        .unwrap_or(0);
    let total = articles.pagination.total_size.unwrap_or(0);
    let (has_prev, has_next) = (offset > 0, offset + count < total);
    let prev_page = if has_prev {
        let offset = offset.saturating_sub(steps);
        Some(format!("{url}steps={steps}&offset={offset}"))
    } else {
        None
    };
    let next_page = if has_next {
        let offset = offset.saturating_add(steps).min(total - 1);
        Some(format!("{url}steps={steps}&offset={offset}"))
    } else {
        None
    };

    let doc = document!(
        "Neuters - Reuters Proxy",
        html! {
            h1 { (title) }
            @if search_type == SearchType::Query {
                form {
                    input type="text" name="query" placeholder="Keywords..." value=(path) required="";
                    button type="submit" { "Search" }
                }
            }
            @if let Some(articles) = articles.articles {
                ul {
                    @for article in articles.iter() {
                        li { a href=(&article.canonical_url) { (&article.title) } }
                    }
                }
                @if has_prev || has_next {
                    div.nav {
                        a href=[prev_page] { "<" }
                        ((offset + 1)) " to " ((offset + count)) " of " (total)
                        a href=[next_page] { ">" }
                    }
                }
            } @else {
                p { "No results found!" }
            }
        },
    );

    Ok(doc.into_string())
}

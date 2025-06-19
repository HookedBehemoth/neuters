use crate::api::{
    common::Articles, error::ApiResult, search::fetch_articles_by_search,
    section::fetch_articles_by_section, topic::fetch_articles_by_topic,
};
use crate::client::Client;
use crate::{document, Section};
use maud::{html, Markup};

#[derive(PartialEq)]
enum SearchType {
    Topic,
    Section,
    Query,
}

pub fn render_topic(client: &Client, path: &str, offset: u32, size: u32) -> ApiResult<String> {
    let article = fetch_articles_by_topic(client, path, offset, size)?;
    let title = article
        .topics
        .as_ref()
        .and_then(|t| t.first().map(|t| t.name.as_str()))
        .unwrap_or("");
    let trailer = html! { h1 { (title) } };
    render_articles(article, path, offset, size, SearchType::Topic, trailer)
}

pub fn render_section(
    client: &Client,
    section: &Section,
    offset: u32,
    size: u32,
) -> ApiResult<String> {
    let article = fetch_articles_by_section(client, &section.id, offset, size)?;
    let trailer = html! {
        div {
            h1 { (section.name) }
            @if !section.children.is_empty() {
                details {
                    summary { "Subsections" }
                    ul {
                        @for child in &section.children {
                            li { a href=(child.id) { (child.name) } }
                        }
                    }
                }
            }
        }
    };
    render_articles(
        article,
        &section.id,
        offset,
        size,
        SearchType::Section,
        trailer,
    )
}

pub fn render_search(client: &Client, request: &rouille::Request) -> ApiResult<String> {
    return Err(crate::api::error::ApiError::Internal("Search is currently disabled due to abuse".to_string()));
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

            let trailer = html! {
                form {
                    input type="text" name="query" placeholder="Keywords..." value=(query) required="";
                    button type="submit" { "Search" }
                }
            };

            render_articles(articles, &query, offset, size, SearchType::Query, trailer)
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
    trailer: Markup,
) -> ApiResult<String> {
    let url = match search_type {
        SearchType::Section => format!("{path}?"),
        SearchType::Topic => format!("{path}?"),
        SearchType::Query => format!("/search?query={path}&"),
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
            (trailer)
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

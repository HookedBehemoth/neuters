use crate::api::{
    common::Articles, error::ApiResult, search::fetch_articles_by_search,
    section::fetch_articles_by_section, topic::fetch_articles_by_topic,
};
use crate::document;
use axum::extract::{OriginalUri, Path, Query, State};
use maud::{html, Markup};
use reqwest::Client;
use serde::Deserialize;

#[derive(PartialEq)]
enum SearchType {
    Topic,
    Section,
    Query,
}

pub async fn topic(client: State<Client>, path: &str, offset: u32, size: u32) -> ApiResult<Markup> {
    let article = fetch_articles_by_topic(&client, path, offset, size).await?;
    render_articles(article, path, offset, size, SearchType::Topic)
}

pub async fn section(
    client: State<Client>,
    Query(id): Query<&str>,
    Query(offset): Query<u32>,
    Query(size): Query<u32>,
) -> ApiResult<Markup> {
    let article = fetch_articles_by_section(&client, &id, offset, size).await?;
    render_articles(article, &id, offset, size, SearchType::Section)
}

#[derive(Deserialize)]
pub struct Paging {
    pub offset: Option<u32>,
    pub size: Option<u32>,
}

pub async fn author(
    client: State<Client>,
    Query(paging): Query<Paging>,
    OriginalUri(uri): OriginalUri) -> ApiResult<Markup> {
    topic(client, uri.path(), paging.offset.unwrap_or(0), paging.size.unwrap_or(20)).await
}

pub async fn home(client: State<Client>) -> ApiResult<Markup> {
    section(client, Query("/home"), Query(0), Query(8)).await
}

#[derive(Deserialize)]
pub struct SearchQuery {
    query: Option<String>,
    offset: Option<u32>,
    size: Option<u32>,
}

pub async fn search(client: State<Client>, Query(search): Query<SearchQuery>) -> ApiResult<Markup> {
    if let Some(query) = search.query {
        let offset = search.offset.unwrap_or(0);
        let size = search.size.unwrap_or(20);

        let articles = fetch_articles_by_search(&client, &query, offset, size).await?;

        render_articles(articles, &query, offset, size, SearchType::Query)
    } else {
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

        Ok(doc)
    }
}

fn render_articles(
    articles: Articles,
    path: &str,
    offset: u32,
    steps: u32,
    search_type: SearchType,
) -> ApiResult<Markup> {
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

    Ok(doc)
}

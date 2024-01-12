use crate::{
    api::{article::fetch_article_by_url, error::ApiResult},
    render::byline,
    routes::HtmxAttributes,
};
use chrono::{DateTime, Utc};
use hypertext::{html_elements, maud, maud_move, GlobalAttributes, Raw, Renderable};

pub fn render_article(client: &ureq::Agent, path: &str) -> ApiResult<String> {
    let article = fetch_article_by_url(client, path)?;

    let published_time = article
        .published_time
        .parse::<DateTime<Utc>>()
        .map(|time| time.format("%Y-%m-%d %H:%M").to_string());

    let doc = crate::document!(
        &article.title,
        maud!(
            h1 { (&article.title) }
            p class="byline" {
                @if let Some(authors) = &article.authors {
                    @if let Ok(time) = &published_time {
                        (time) " - "
                    }
                    (byline::render_byline(authors))
                }
            }
            @if let Some(articles) = &article.content_elements {
                (render_items(articles))
            }
        ),
        maud! {
            meta property="og:title" content=(&article.title);
            meta property="og:type" content="article";
            meta property="og:description" content=(&article.description);
            meta property="og:url" content=(path);
        }
    );

    Ok(doc.render().into_inner())
}

fn render_items(items: &[serde_json::Value]) -> impl Renderable + '_ {
    maud_move! {
        @for content in items {
            @match content["type"].as_str() {
                Some("header") => {
                    @if let Some(header) = content["content"].as_str() {
                        @match content["level"].as_u64().unwrap_or(0) {
                            0 => h1 { (header) },
                            1 => h2 { (header) },
                            _ => h3 { (header) },
                        }
                    }
                }
                Some("paragraph") => {
                    @if let Some(content) = content["content"].as_str() {
                        p { (Raw(&content)) }
                    }
                }
                Some("image") => {
                    @if let Some(image) = content["url"].as_str() {
                        @let alt = content["alt"].as_str();
                        @let (width, height) = (content["width"].as_u64(), content["height"].as_u64());
                        img src=(image) alt=[alt] width=[width] height=[height];
                    }
                }
                Some("graphic") => {
                    @match content["graphic_type"].as_str() {
                        Some("image") => {
                            @if let (Some(image), Some(description)) = (content["url"].as_str(), content["description"].as_str()) {
                                figure {
                                    img src=(image) alt=(description);
                                    figcaption { (description) }
                                }
                            }
                        }
                        Some(unknown) => { p { "Unknown graphic type: " (unknown) } }
                        None => { p { "Missing graphic type" } }
                    }
                }
                Some("table") => {
                    @let rows = match content["rows"].as_array() { Some(rows) => rows, None => continue };
                    table {
                        thead {
                            @let row = match rows[0].as_array() { Some(row) => row, None => continue };
                            tr {
                                @for cell in row.iter() {
                                    th { (cell.as_str().unwrap_or_default()) }
                                }
                            }
                        }
                        tbody {
                            @for row in rows[1..].iter() {
                                tr {
                                    @let cells = match row.as_array() { Some(cells) => cells, None => continue };
                                    @for cell in cells {
                                        td { (Raw(cell.as_str().unwrap_or_default())) }
                                    }
                                }
                            }
                        }
                    }
                }
                Some("list") => {
                    @if let Some(items) = content["items"].as_array() {
                        (render_items(items))
                    }
                }
                Some("social_media") => {
                    @if let Some(markup) = content["html"].as_str() {
                        @let embed = if let Some(index) = markup.find("\n<script") {
                            &markup[..index]
                        } else {
                            markup
                        };
                       (Raw(embed))
                    }
                }
                Some(unknown) => { p { "Unknown type: " (unknown) } }
                None => { p { "Failed to parse content element" } }
            }
        }
    }
}

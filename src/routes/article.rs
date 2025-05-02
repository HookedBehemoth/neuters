use crate::{
    api::{article::fetch_article_by_url, common::Image, error::ApiResult},
    client::Client,
    render::{byline, images::render_image},
    settings::Settings,
};
use chrono::{DateTime, Utc};
use maud::{html, PreEscaped};

pub fn render_article(client: &Client, path: &str, settings: &Settings) -> ApiResult<String> {
    let article = fetch_article_by_url(client, path)?;

    let published_time = article
        .published_time
        .parse::<DateTime<Utc>>()
        .map(|time| time.format("%Y-%m-%d %H:%M").to_string());

    let doc = crate::document!(
        &article.title,
        html!(
            h1 { (&article.title) }
            p class="byline" {
                @if let Some(authors) = &article.authors {
                    @let byline = byline::render_byline(authors);
                    @if let Ok(time) = published_time {
                        (time) " - "
                    }
                    (PreEscaped(byline))
                }
            }
            @if Some("live-blog") == article.subtype.as_deref() {
                p {
                    i {
                        "You seem to have accidentally clicked on AI sloppa. There is nothing of value here."
                    }
                }
                p {
                    i {
                        "Neuters is currently not planning to support live blogs.
                        If you want to see the original \"content\", disable any redirector extension and click on this link: "
                        @let url = format!("https://www.reuters.com{}", path);
                        a href=(url) { "Original" }
                    }
                }
            } @else {
                @if settings.embed_images {
                    @if let Some(thumbnail) = &article.thumbnail {
                        (render_image(thumbnail, settings))
                    }
                }
                (render_items(&article.content_elements.unwrap_or_default(), settings))
            }
        ),
        html! {
            meta property="og:title" content=(&article.title);
            meta property="og:type" content="article";
            meta property="og:description" content=(&article.description);
            meta property="og:url" content=(path);
        }
    );

    Ok(doc.into_string())
}

fn render_items(items: &[serde_json::Value], settings: &Settings) -> maud::Markup {
    html! {
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
                        @if content.contains("href=\"https://www.reuters.com/") {
                            p { (PreEscaped(content.replace("href=\"https://www.reuters.com/", "href=\"/"))) }
                        } @else {
                            p { (PreEscaped(content)) }
                        };
                    }
                }
                Some("image") => {
                    @if settings.embed_images {
                        @if let Some(image) = content["url"].as_str() {
                            @let alt = content["alt"].as_str();
                            @let (width, height) = (content["width"].as_u64(), content["height"].as_u64());
                            img src=(image) alt=[alt] width=[width] height=[height];
                        }
                    } @else {
                        p {
                            i {
                                "Embedding images is disabled. Navigate to the original resource or change the settings to enable it."
                            }
                        }
                        @if let Some(image) = content["url"].as_str() {
                            p {
                                a href=(image) { "Image" }
                            }
                        }
                    }
                }
                Some("graphic") => {
                    @if settings.embed_images {
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
                    } @else {
                        p {
                            i {
                                "Embedding images is disabled. Navigate to the original resource or change the settings to enable it."
                            }
                        }
                        @if let Some(image) = content["url"].as_str() {
                            p {
                                a href=(image) { "Image" }
                            }
                        }
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
                                        td { (PreEscaped(cell.as_str().unwrap_or_default())) }
                                    }
                                }
                            }
                        }
                    }
                }
                Some("list") => {
                    @if let Some(items) = content["items"].as_array() {
                        (render_items(items, settings))
                    }
                }
                Some("social_media") => {
                    @if settings.embed_embeds {
                        @if let Some(markup) = content["html"].as_str() {
                            @let embed = if let Some(index) = markup.find("\n<script") {
                                &markup[..index]
                            } else {
                                markup
                            };
                           (maud::PreEscaped(embed))
                        }
                    } @else {
                        p {
                            i {
                                "Embedding social media is disabled. Navigate to the original resource or change the settings to enable it."
                            }
                        }
                    }
                }
                Some(unknown) => { p { "Unknown type: " (unknown) } }
                None => { p { "Failed to parse content element" } }
            }
        }
    }
}

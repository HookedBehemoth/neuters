use std::fmt::Write;

use crate::api::internet_news::InternetNewsAuthor;

pub fn render_byline(authors: &[InternetNewsAuthor]) -> String {
    match authors.len() {
        0 => "".to_string(),
        1 => format_author(&authors[0]),
        author_count => {
            /* Chain author names together */
            let mut byline = "By ".to_string();

            for author in authors[..author_count - 2].iter() {
                byline.push_str(&format_author(author));
                byline.push_str(", ");
            }

            let _ = write!(
                byline,
                "{} and {}",
                format_author(&authors[author_count - 2]),
                format_author(&authors[author_count - 1])
            );

            byline
        }
    }
}

pub fn format_author(author: &InternetNewsAuthor) -> String {
    match author
        .url
        .strip_prefix("https://www.reuters.com/journalists/")
    {
        Some(path) => format!("<a href=\"/authors/{}/\">{}</a>", path, author.name),
        None => format!("<a href=\"{}\">{}</a>", author.url, author.name),
    }
}

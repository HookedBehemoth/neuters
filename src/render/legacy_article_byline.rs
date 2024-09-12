use hypertext::{html_elements, maud, Renderable};

use crate::api::legacy_article::LegacyArticleAuthor;

pub fn render_byline(authors: &[LegacyArticleAuthor]) -> impl Renderable + '_ {
    maud! {
        @match authors.len() {
            0 => {},
            1 => (format_author(&authors[0])),
            author_count => {
                "By "
                @for author in &authors[..author_count - 2] {
                    (format_author(author)) ", "
                }
                (format_author(&authors[author_count - 2]))
                " and "
                (format_author(&authors[author_count - 1]))
            }
        }
    }
}

pub fn format_author(author: &LegacyArticleAuthor) -> impl Renderable + '_ {
    maud! {
        a href=@if let Some(path) = author
                .url
                .strip_prefix("https://www.reuters.com/journalists/")
            {
                "/journalists/" (path) "/"
            } @else {
                (&author.url)
            }
        {
            (&author.name)
        }
    }
}

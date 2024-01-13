use hypertext::{maud, Renderable};

use crate::api::common::Topic;
use hypertext::html_elements;

pub fn render_byline(authors: &[Topic]) -> impl Renderable + '_ {
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

pub fn format_author(author: &Topic) -> impl Renderable + '_ {
    maud! {
        @if let Some(url) = &author.topic_url {
            a href=(url) {
                (&author.byline)
            }
        } @else {
            (&author.byline)
        }
    }
}

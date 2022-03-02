use super::common::Topic;

pub fn render_byline(authors: &[Topic]) -> String {
    let author_count = authors.len();

    if author_count == 1 {
        format_author(&authors[0])
    } else {
        /* Chain author names together */
        let mut byline = "By ".to_string();

        for author in authors[..author_count - 2].iter() {
            byline.push_str(&format_author(author));
            byline.push_str(", ");
        }

        byline.push_str(&format!(
            "{} and {}",
            format_author(&authors[author_count - 2]),
            format_author(&authors[author_count - 1])
        ));

        byline
    }
}

pub fn format_author(author: &Topic) -> String {
    format!("<a href=\"{}\">{}</a>", author.topic_url, author.byline)
}

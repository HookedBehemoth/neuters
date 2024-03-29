use crate::api::error::ApiResult;
use crate::document;
use maud::html;
use std::env;

const GIT_HASH: &str = env!("GIT_HASH");

pub fn render_about() -> ApiResult<String> {
    let doc = document!(
        "About",
        html! {
            h1 { "About" }
            p { "This is an alternative frontend to " a href="https://www.reuters.com/" { "Reuters" } ". It is intented to be lightweight, fast and was heavily inspired by " a href="https://nitter.net/" { "Nitter" } "." }
            ul {
                li { "No JavaScript or ads" }
                li { "No tracking" }
                li { "No cookies" }
                li { "Lightweight (usually <10KiB vs 50MiB from Reuters)" }
                li { "Dynamic Theming (respects system theme)" }
            }
            p { "You can install " a href="https://libredirect.github.io/" { "libredirect" } " or " a href="https://addons.mozilla.org/en-US/firefox/addon/reuters-redirect/" { "this browser extension" } " to automatically forwards all reuters links to this site." }
            p { "This is a work in progress. Please report any bugs or suggestions at " a href="https://github.com/HookedBehemoth/neuters" { "GitHub" } "." }

            h2 { "Contact" }
            p { "If you have any questions, feel free to contact me at " a href = "mailto:admin@boxcat.site" { "admin@boxcat.site" } "." }

            h2 { "Credits" }
            ul {
                li { a href="https://github.com/lambda-fairy/maud" { "maud" } ", a fast and intuitive inline html macro" }
            }

            h2 { "License" }
            p { "This project is licensed under the " a href="https://www.gnu.org/licenses/licenses.html#AGPL" { "GNU Affero General Public License" } "." }

            h2 { "Build information" }
            p { "This version is based off the git commit " a href=(format!("https://github.com/HookedBehemoth/neuters/commit/{}", GIT_HASH)) { (GIT_HASH) }}
        },
    );

    Ok(doc.into_string())
}

use maud::{html, Markup};
use rouille::{post_input, try_or_400, Request, Response};

use crate::{
    document,
    settings::{Settings, EMBED_EMBEDS, EMBED_IMAGES, FAST_REDIRECT, PROXY_IMAGES, REDIRECT_TIMER, ARTICLE_LIMIT},
};

fn render_settings(
    embed_images: bool,
    embed_embeds: bool,
    proxy_images: bool,
    fast_redirect: bool,
    redirect_timer: u32,
    article_limit: u32,
) -> Markup {
    document!(
        "Settings",
        html! {
            h1 { "Settings" }
            p {
                "Settings will be stored in the browsers cookie storage and transferred to the server on each request. This site only uses cookies to store preferences and does not track you."
            }
            form method="POST" {
                label for=(EMBED_IMAGES) {
                    "Embed images in articles"
                    input type="checkbox" id=(EMBED_IMAGES) name=(EMBED_IMAGES) checked[embed_images] {}
                }

                label for=(EMBED_EMBEDS) {
                    "Embed embeds in articles"
                    input type="checkbox" id=(EMBED_EMBEDS) name=(EMBED_EMBEDS) checked[embed_embeds] {}
                }

                label for=(PROXY_IMAGES) {
                    "Proxy images through the server"
                    input type="checkbox" id=(PROXY_IMAGES) name=(PROXY_IMAGES) checked[proxy_images] {}
                }

                label for=(FAST_REDIRECT) {
                    "Use fast redirect"
                    input type="checkbox" id=(FAST_REDIRECT) name=(FAST_REDIRECT) checked[fast_redirect] {}
                }

                label for=(REDIRECT_TIMER) {
                    "Redirect timer"
                    input type="number" id=(REDIRECT_TIMER) name=(REDIRECT_TIMER) value=(redirect_timer) {}
                }

                label for=(ARTICLE_LIMIT) {
                    "Articles to display"
                    input type="number" id=(ARTICLE_LIMIT) name=(ARTICLE_LIMIT) value=(article_limit) {}
                }                

                button type="submit" {
                    "Save"
                }
            }
        },
    )
}

fn store_settings(
    embed_images: bool,
    embed_embeds: bool,
    proxy_images: bool,
    fast_redirect: bool,
    redirect_timer: u32,
    article_limit: u32,
) -> Response {
    Response::redirect_303("/settings")
        .with_additional_header(
            "Set-Cookie",
            format!("{EMBED_IMAGES}={}; Path=/; SameSite=Strict", embed_images),
        )
        .with_additional_header(
            "Set-Cookie",
            format!("{EMBED_EMBEDS}={}; Path=/; SameSite=Strict", embed_embeds),
        )
        .with_additional_header(
            "Set-Cookie",
            format!("{PROXY_IMAGES}={}; Path=/; SameSite=Strict", proxy_images),
        )
        .with_additional_header(
            "Set-Cookie",
            format!("{FAST_REDIRECT}={}; Path=/; SameSite=Strict", fast_redirect),
        )  
        .with_additional_header(
            "Set-Cookie",
            format!(
                "{REDIRECT_TIMER}={}; Path=/; SameSite=Strict",
                redirect_timer
            ),
        )
        .with_additional_header(
            "Set-Cookie",
            format!(
                "{ARTICLE_LIMIT}={}; Path=/; SameSite=Strict",
                article_limit
            ),
        )           
}

pub fn handle_settings(request: &Request, settings: &Settings) -> Response {
    if request.method() == "POST" {
        let settings = try_or_400!(post_input!(request, {
            embed_images: bool,
            embed_embeds: bool,
            proxy_images: bool,
            fast_redirect: bool,
            redirect_timer: i32,
            article_limit: i32,
        }));

        store_settings(
            settings.embed_images,
            settings.embed_embeds,
            settings.proxy_images,
            settings.fast_redirect,
            settings.redirect_timer.clamp(0, 600) as u32,
            settings.article_limit.clamp(0, 100) as u32,
        )
    } else {
        let page = render_settings(
            settings.embed_images,
            settings.embed_embeds,
            settings.proxy_images,
            settings.fast_redirect,
            settings.redirect_timer,
            settings.article_limit,
        );
        Response::html(page).with_status_code(200)
    }
}

use rouille::{input, Request};

pub const EMBED_IMAGES: &str = "embed_images";
pub const EMBED_EMBEDS: &str = "embed_embeds";
pub const PROXY_IMAGES: &str = "proxy_images";
pub const FAST_REDIRECT: &str = "fast_redirect";
pub const REDIRECT_TIMER: &str = "redirect_timer";

pub struct Settings {
    pub embed_images: bool,
    pub embed_embeds: bool,
    pub proxy_images: bool,
    pub fast_redirect: bool,
    pub redirect_timer: u32,
}

impl Settings {
    pub fn from_request(request: &Request) -> Self {
        let mut embed_images = true;
        let mut embed_embeds = true;
        let mut proxy_images = false;
        let mut fast_redirect = false;
        let mut redirect_timer = 5;
        for (key, value) in input::cookies(request) {
            match key {
                EMBED_IMAGES => embed_images = value == "true",
                EMBED_EMBEDS => embed_embeds = value == "true",
                PROXY_IMAGES => proxy_images = value == "true",
                FAST_REDIRECT => fast_redirect = value == "true",
                REDIRECT_TIMER => redirect_timer = value.parse().unwrap_or(5),
                _ => {}
            }
        }

        Self {
            embed_images,
            embed_embeds,
            proxy_images,
            fast_redirect,
            redirect_timer,
        }
    }
}
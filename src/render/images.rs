use crate::{api::common::Image, routes::proxy, settings::Settings};

const RESIZE_STEPS: [u16; 6] = [480, 640, 720, 960, 1080, 1200];

pub fn render_image(thumbnail: &Image, settings: &Settings) -> maud::Markup {
    let resizer_url = &thumbnail.resizer_url;

    let url = if settings.proxy_images {
        if let Some(base_path) = proxy::strip_prefix(resizer_url) {
            format!("/proxy/{base_path}")
        } else {
            return maud::html! {
                p {
                    i { "Proxy requested but no supported link found!" }
                }
            };
        }
    } else {
        resizer_url.to_string()
    };
    let mut srcset = String::new();
    for width in RESIZE_STEPS {
        if width > thumbnail.width {
            break;
        }
        srcset.push_str(&format!("{url}&width={width}&quality=80 {width}w,"));
    }

    maud::html! {
        figure {
            img src=(url)
                srcset=(srcset)
                width=(thumbnail.width) height=(thumbnail.height)
                alt="";
            @if let Some(caption) = &thumbnail.caption {
                figcaption { i { (caption) } }
            }
        }
    }
}

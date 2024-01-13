use hypertext::{Attribute, GlobalAttributes};

pub mod about;
pub mod article;
pub mod internet_news;
pub mod markets;
pub mod search;

trait HtmxAttributes: GlobalAttributes {
    #[allow(non_upper_case_globals)]
    const property: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}

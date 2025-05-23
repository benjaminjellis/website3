use std::sync::LazyLock;

use maud::Markup;

mod pocari;

pub struct BlogPost {
    pub title: &'static str,
    pub url: &'static str,
    #[allow(dead_code)]
    pub html: maud::Markup,
}

impl BlogPost {
    fn new(page: fn() -> (&'static str, &'static str, Markup)) -> Self {
        let (title, url, html) = page();
        Self { url, html, title }
    }
}

pub(crate) static BLOG_POSTS: LazyLock<[BlogPost; 1]> =
    LazyLock::new(|| [BlogPost::new(pocari::blog_post)]);

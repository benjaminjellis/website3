use crate::shared::layout;
use maud::{Markup, html};

pub(in crate::blog_posts) fn blog_post() -> (&'static str, &'static str, Markup) {
    let title = "Test Blog";
    let post = html! {};
    let markup = layout(title, post);
    (title, "test_blog", markup)
}

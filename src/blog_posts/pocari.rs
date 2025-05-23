use maud::{Markup, PreEscaped, html};

pub(in crate::blog_posts) fn blog_post() -> (&'static str, &'static str, Markup) {
    let title = "A (not yet) comprehensive blog on where to find pocari sweat in london";
    let post = html! {
        h1 { (PreEscaped(title)) }
    };
    (title, "where_to_find_pocari_sweat_in_london", post)
}

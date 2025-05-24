use axum::extract::Path;
use maud::{Markup, html};

use crate::{
    blog_posts::BLOG_POSTS,
    shared::{four_oh_four, layout},
};

pub(crate) async fn index() -> Markup {
    let content = html! {
        h2."font-bold" {
            bold{ "about me:" }
        }
        p {
            "in my day to day I'm a backend engineer, primarily writing Rust at a stealth fintech startup"
        }
        br;
        h2."font-bold" {
            "some projects of varying usefulnesss:"
        }
        ul."list-disc list-inside"{
            li."italic"{ a."hover:underline" href="https://github.com/benjaminjellis/gegen"{"gegen: football scores and fixtures from across the world, in the terminal "}}
            li."italic"{ a."hover:underline" href="https://github.com/benjaminjellis/cherry2"{"cherry: a a coffee logbook to help you keep track of your brews (currently WIP)"}}
        }
        br;
        h2."font-bold"{
            "current:"
        }
        ul class="list-disc list-inside"{
            li {"backend engineer @ stealth fintech startup"}
        }
        br;
        h2."font-bold"{
            "previous:"
        }
        ul class="list-disc list-inside"{
            li {"data scientist @ Capgemini Invent"}
            li {"financial engineer @ IHS Markit"}
            li {"consultant @ EY"}
        }
    };

    layout("benjamin ellis", &content)
}

pub(crate) async fn blog_overview() -> Markup {
    let post = BLOG_POSTS
        .iter()
        .map(|post| (post.title, post.url))
        .collect::<Vec<_>>();

    let blog_posts = html! {
        ul class="list-disc list-inside"{
        @for (titie, url) in &post{
            @let url = format!("/blog/{url}");
            li { a href=(url) { (titie) } }
        }
     }
    };

    layout("blog", &blog_posts)
}

pub(crate) async fn blog_post(Path(post_name): Path<String>) -> Markup {
    match BLOG_POSTS.iter().find(|post| post.url == post_name) {
        Some(blog_post) => layout(blog_post.title, &blog_post.html),
        None => four_oh_four(),
    }
}

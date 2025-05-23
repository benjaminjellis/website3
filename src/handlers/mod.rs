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
            "in my day to day I'm a backend engineer working in Rust at a stealth fintech startup"
        }
        br;
        h2."font-bold" {
            "outside of work I:"
        }
        ul class="list-disc list-inside"{
            li {"write even more rust (this entire website is written in rust)" }
            li {"dabble in some functional programming (I really like gleam right now)"}
            li {"make ok pourover coffee"}
            li {"lose many an online chess game"}
            li {"watch Arsenal"}
        }
        br;
        h2."font-bold"{
            "current:"
        }
        ul class="list-disc list-inside"{
            li {"backend engineer (Rust) @ stealth fintech startup"}
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
        br;
        h2."font-bold"{
            "education:"
        }
        ul class="list-disc list-inside"{
            li {"University College London"}
            li {"University of Liverpool"}
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
        Some(blog_post) => layout("blog post", &blog_post.html),
        None => four_oh_four(),
    }
}

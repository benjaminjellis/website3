use maud::{Markup, PreEscaped, html};

fn stockist(name: &str, link: &str) -> Markup {
    html! {li."italic hover:underline"{a href=(link){(name)}}}
}

pub(in crate::blog_posts) fn blog_post() -> (&'static str, &'static str, Markup) {
    let title = "A (not yet) comprehensive blog on where to find pocari sweat in london";
    let post = html! {
        h1."text-xl font-bold"{ (PreEscaped(title)) }
        br;
        p {"While ubiquitous across Asia, Pocari Sweat isn't as easy to find in London (where I happen to live). But that doesn't mean it can't be found."}
        p {"Below is a list of all the stockists that I have found so far."}
        p {"I'll endeavor to update as I find more, but expect this list to be heavily weighted towards Hackney and Bloomsbury, given that's where I spend most of my time."}
        br;
        ul."list-disc list-inside"{
            (stockist("Oseyo (Waterloo)", "https://g.co/kgs/B2RCbGm"))
            (stockist("Oseyo (Tottenham Court Road)", "https://g.co/kgs/obkpPZe"))
            (stockist("Centre Point Food Store (Tottenham Court Road)", "https://g.co/kgs/ijMbsBg"))
            (stockist("Longdan (Shoreditch)", "https://g.co/kgs/x928kw1"))
        }
    };
    (title, "where_to_find_pocari_sweat_in_london", post)
}

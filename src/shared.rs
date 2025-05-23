use maud::{DOCTYPE, Markup, html};

fn nav_item(link: &str, text: &str) -> Markup {
    html! {
        li{a."block py-2 text-gray-800 hover:underline text-center font-bold" href=(link) {(text)}}
    }
}

pub(crate) fn four_oh_four() -> Markup {
    layout(
        "uh oh :(",
        html!(
            p{"we coulnd't find"}
        ),
    )
}

pub(crate) fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link href="/static/output.css" rel="stylesheet";
            title {
                (title)
            }
        }
        body."flex h-screen"{
            aside."bg-yellow-300 w-1/10 flex flex-col"{
                nav."h-4/5 p-8"{
                   ul."space-y-1" {
                        (nav_item("/", "home"))
                        (nav_item("/blog", "blog"))
                        (nav_item("https://github.com/benjaminjellis", "github"))
                        (nav_item("https://lichess.org/@/agnesmartinstan", "lichess"))
                        (nav_item("https://lichess.org/@/agnesmartinstan", "linkedin"))
                    }
                }
               div."h-1/5 p-4 flex items-center justify-center"{
                    h1."text-2xl font-bold text-gray-900 text-center leading-tight;"{
                        "Benjamin Ellis"
                    }
                }
            }
            main."flex-1 p-8 bg-yellow-100"{
                (content)
            }
        }

    }
}

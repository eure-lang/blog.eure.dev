use maud::{html, Markup, DOCTYPE};

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | Eure Blog" }
                link rel="stylesheet" href="/styles/main.css";
                link rel="stylesheet" href="/styles/syntax.css";
                link rel="stylesheet" href="/styles/eure-syntax.css";
            }
            body {
                header.site-header {
                    nav.site-nav {
                        a.site-title href="/" { "Eure Blog" }
                    }
                }
                main.site-main { (content) }
                footer.site-footer {
                    p { "Powered by Eure" }
                }
            }
        }
    }
}

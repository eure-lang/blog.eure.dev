use maud::{html, Markup, DOCTYPE};

pub const BASE_URL: &str = "https://blog.eure.dev";
pub const SITE_NAME: &str = "Eure Blog";
pub const DEFAULT_DESCRIPTION: &str = "A blog about Eure, a data notation language for algebraic data types and deeply nested data.";

pub struct OgpMeta<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub url: &'a str,
    pub og_type: &'a str,
}

pub fn base_layout(title: &str, content: Markup, ogp: &OgpMeta) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | " (SITE_NAME) }
                meta name="description" content=(ogp.description);
                // Open Graph
                meta property="og:title" content=(ogp.title);
                meta property="og:description" content=(ogp.description);
                meta property="og:url" content=(ogp.url);
                meta property="og:type" content=(ogp.og_type);
                meta property="og:site_name" content=(SITE_NAME);
                meta property="og:image" content=(format!("{}/ogp.png", BASE_URL));
                // Twitter Card
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content=(ogp.title);
                meta name="twitter:description" content=(ogp.description);
                meta name="twitter:image" content=(format!("{}/ogp.png", BASE_URL));
                // Favicon
                link rel="icon" type="image/x-icon" href="/favicon.ico";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="manifest" href="/site.webmanifest";
                // Stylesheets
                link rel="stylesheet" href="/styles/main.css";
                link rel="stylesheet" href="/styles/syntax.css";
                link rel="stylesheet" href="/styles/eure-syntax.css";
            }
            body {
                header.site-header {
                    nav.site-nav {
                        div.header-left {
                            a.eure-logo href="https://eure.dev" target="_blank" {
                                img src="https://raw.githubusercontent.com/Hihaheho/eure/main/assets/eure-logo-dark.svg" alt="Eure" height="32";
                            }
                            span.divider {}
                            a.site-title href="/" { "Blog" }
                        }
                        div.header-nav-links {
                            a.github-link href="https://github.com/Hihaheho/eure" target="_blank" {
                                (maud::PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 16 16"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>"#))
                            }
                        }
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

use maud::{Markup, html};

use crate::render::code_highlight::CodeHighlighter;
use crate::render::eure_highlight::render_eure_highlighted_with_line_numbers;
use crate::templates::base::{base_layout, OgpMeta, BASE_URL, DEFAULT_DESCRIPTION};

const GITHUB_REPO: &str = "eure-lang/blog.eure.dev";

pub fn render_source_page(
    slug: &str,
    title: &str,
    source_content: &str,
    commit_hash: Option<&str>,
    highlighter: &CodeHighlighter,
) -> Markup {
    let github_url = commit_hash.map(|hash| {
        format!(
            "https://github.com/{}/blob/{}/articles/{}.eure",
            GITHUB_REPO, hash, slug
        )
    });

    let content = html! {
        article.source-view {
            header.source-header {
                h1.source-title { "Source: " (title) }
                div.source-actions {
                    a.source-back-link href=(format!("/articles/{}.html", slug)) {
                        "← Back to article"
                    }
                    @if let Some(url) = &github_url {
                        a.source-github-link href=(url) target="_blank" rel="noopener noreferrer" {
                            "GitHub"
                        }
                    }
                }
            }
            div.source-content {
                (render_eure_highlighted_with_line_numbers(source_content, Some(highlighter)))
            }
        }
    };

    let url = format!("{}/source/{}.html", BASE_URL, slug);
    let ogp = OgpMeta {
        title: &format!("Source: {}", title),
        description: DEFAULT_DESCRIPTION,
        url: &url,
        og_type: "website",
    };
    base_layout(&format!("Source: {}", title), content, &ogp)
}

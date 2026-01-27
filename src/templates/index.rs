use maud::{html, Markup};

use crate::article::Article;
use crate::templates::base_layout;

pub struct ArticleEntry<'a> {
    pub slug: &'a str,
    pub article: &'a Article,
}

pub fn render_index_page(articles: &[ArticleEntry]) -> Markup {
    let content = html! {
        div.index-page {
            h1.page-title { "Articles" }
            ul.article-list {
                @for entry in articles {
                    li.article-list-item {
                        a.article-link href=(format!("/articles/{}.html", entry.slug)) {
                            span.article-title { (entry.article.frontmatter.title.as_str()) }
                        }
                        @if let Some(date) = &entry.article.frontmatter.date {
                            time.article-date { (date.as_str()) }
                        }
                        @if !entry.article.frontmatter.tags.is_empty() {
                            div.article-tags {
                                @for tag in &entry.article.frontmatter.tags {
                                    span.article-tag { (tag) }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    base_layout("Home", content)
}

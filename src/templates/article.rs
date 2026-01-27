use eure::value::Text;
use indexmap::IndexMap;
use maud::{html, Markup};

use crate::article::{Article, Item, Level2, Level3, Level4, Level5, Level6, TextOrNested};
use crate::render::{render_text, CodeHighlighter};
use crate::templates::base_layout;

pub fn render_article_page(article: &Article, highlighter: &CodeHighlighter) -> Markup {
    let content = html! {
        article.article {
            header.article-header {
                h1.article-title { (render_text(&article.header, highlighter)) }
                @if let Some(date) = &article.frontmatter.date {
                    div.article-meta {
                        time.article-date { (date.as_str()) }
                    }
                }
                @if !article.frontmatter.tags.is_empty() {
                    div.article-tags {
                        @for tag in &article.frontmatter.tags {
                            span.article-tag { (tag) }
                        }
                    }
                }
            }
            div.article-content {
                @for (key, item) in &article.sections {
                    (render_item(key, item, highlighter))
                }
            }
        }
    };

    base_layout(article.frontmatter.title.as_str(), content)
}

fn render_item<T>(key: &str, item: &Item<T>, highlighter: &CodeHighlighter) -> Markup
where
    T: RenderNested,
{
    match item {
        Item::Normal(value) => html! {
            div.content-item data-key=(key) {
                (value.render(highlighter))
            }
        },
        Item::List(items) => html! {
            div.content-list data-key=(key) {
                @for value in items {
                    div.content-list-item {
                        (value.render(highlighter))
                    }
                }
            }
        },
    }
}

trait RenderNested {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup;
}

impl RenderNested for Text {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        render_text(self, highlighter)
    }
}

impl<T: RenderNested> RenderNested for TextOrNested<T> {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        match self {
            TextOrNested::Text(text) => render_text(text, highlighter),
            TextOrNested::Nested(nested) => nested.render(highlighter),
        }
    }
}

impl RenderNested for Level2 {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        render_section(&self.header, &self.sections, "h2", highlighter)
    }
}

impl RenderNested for Level3 {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        render_section(&self.header, &self.sections, "h3", highlighter)
    }
}

impl RenderNested for Level4 {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        render_section(&self.header, &self.sections, "h4", highlighter)
    }
}

impl RenderNested for Level5 {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        render_section(&self.header, &self.sections, "h5", highlighter)
    }
}

impl RenderNested for Level6 {
    fn render(&self, highlighter: &CodeHighlighter) -> Markup {
        render_section(&self.header, &self.sections, "h6", highlighter)
    }
}

fn render_section<T: RenderNested>(
    header: &Text,
    sections: &IndexMap<String, Item<T>>,
    level: &str,
    highlighter: &CodeHighlighter,
) -> Markup {
    let section_class = format!("article-section article-section-{}", level);
    html! {
        section class=(section_class) {
            @match level {
                "h2" => h2.section-header { (render_text(header, highlighter)) },
                "h3" => h3.section-header { (render_text(header, highlighter)) },
                "h4" => h4.section-header { (render_text(header, highlighter)) },
                "h5" => h5.section-header { (render_text(header, highlighter)) },
                "h6" => h6.section-header { (render_text(header, highlighter)) },
                _ => h2.section-header { (render_text(header, highlighter)) },
            }
            @for (key, item) in sections {
                (render_item(key, item, highlighter))
            }
        }
    }
}

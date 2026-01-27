use std::collections::HashSet;

use eure::value::Text;
use indexmap::IndexMap;
use maud::{html, Markup};

use crate::article::{Article, Item, Level2, Level3, Level4, Level5, Level6, TextOrNested, TocEntry};
use crate::render::{render_text, CodeHighlighter};
use crate::templates::base_layout;

pub fn render_article_page(article: &Article, highlighter: &CodeHighlighter) -> Result<Markup, String> {
    // Collect TOC entries and validate unique IDs
    let mut seen_ids = HashSet::new();
    let toc_entries = collect_toc_entries(&article.sections, &mut seen_ids)?;

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
                    (render_item_with_id(key, item, highlighter, &toc_entries))
                }
            }
        }
    };

    Ok(base_layout(article.frontmatter.title.as_str(), content))
}

/// Extract plain text from a Text value (strips any formatting)
fn extract_plain_text(text: &Text) -> String {
    text.as_str().to_string()
}

/// Collect TOC entries from article sections, validating uniqueness
fn collect_toc_entries<T: CollectToc>(
    sections: &IndexMap<String, Item<TextOrNested<T>>>,
    seen_ids: &mut HashSet<String>,
) -> Result<Vec<TocEntry>, String> {
    let mut entries = Vec::new();

    for (id, item) in sections {
        match item {
            Item::Normal(value) => {
                if let Some(entry) = value.collect_toc_entry(id, seen_ids)? {
                    entries.push(entry);
                }
            }
            Item::List(items) => {
                for value in items {
                    if let Some(entry) = value.collect_toc_entry(id, seen_ids)? {
                        entries.push(entry);
                    }
                }
            }
            Item::Toc(_) => {
                // TOC marker doesn't generate entries
            }
        }
    }

    Ok(entries)
}

/// Trait for collecting TOC entries
trait CollectToc {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String>;
}

impl CollectToc for Text {
    fn collect_toc_entry(&self, _id: &str, _seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        // Text nodes don't generate TOC entries
        Ok(None)
    }
}

impl<T: CollectToc> CollectToc for TextOrNested<T> {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        match self {
            TextOrNested::Text(_) => Ok(None),
            TextOrNested::Nested(nested) => nested.collect_toc_entry(id, seen_ids),
        }
    }
}

impl CollectToc for Level2 {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        if !seen_ids.insert(id.to_string()) {
            return Err(format!("Duplicate section ID: {}", id));
        }
        let children = collect_toc_entries(&self.sections, seen_ids)?;
        Ok(Some(TocEntry {
            id: id.to_string(),
            title: extract_plain_text(&self.header),
            level: 2,
            children,
        }))
    }
}

impl CollectToc for Level3 {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        if !seen_ids.insert(id.to_string()) {
            return Err(format!("Duplicate section ID: {}", id));
        }
        let children = collect_toc_entries(&self.sections, seen_ids)?;
        Ok(Some(TocEntry {
            id: id.to_string(),
            title: extract_plain_text(&self.header),
            level: 3,
            children,
        }))
    }
}

impl CollectToc for Level4 {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        if !seen_ids.insert(id.to_string()) {
            return Err(format!("Duplicate section ID: {}", id));
        }
        // Level 4+ are not included in TOC but still need unique IDs
        // Recursively check children for duplicate IDs
        collect_toc_entries(&self.sections, seen_ids)?;
        Ok(None)
    }
}

impl CollectToc for Level5 {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        if !seen_ids.insert(id.to_string()) {
            return Err(format!("Duplicate section ID: {}", id));
        }
        collect_toc_entries(&self.sections, seen_ids)?;
        Ok(None)
    }
}

impl CollectToc for Level6 {
    fn collect_toc_entry(&self, id: &str, seen_ids: &mut HashSet<String>) -> Result<Option<TocEntry>, String> {
        if !seen_ids.insert(id.to_string()) {
            return Err(format!("Duplicate section ID: {}", id));
        }
        // Level6 sections contain Text, check those for duplicate IDs
        for (child_id, _) in &self.sections {
            if !seen_ids.insert(child_id.to_string()) {
                return Err(format!("Duplicate section ID: {}", child_id));
            }
        }
        Ok(None)
    }
}

/// Render the Table of Contents
fn render_toc(entries: &[TocEntry]) -> Markup {
    html! {
        details.article-toc open {
            summary { "Table of Contents" }
            nav {
                (render_toc_list(entries))
            }
        }
    }
}

fn render_toc_list(entries: &[TocEntry]) -> Markup {
    html! {
        ul {
            @for entry in entries {
                li {
                    a href=(format!("#{}", entry.id)) { (entry.title) }
                    @if !entry.children.is_empty() {
                        (render_toc_list(&entry.children))
                    }
                }
            }
        }
    }
}

/// Render an item with ID support for nested sections
fn render_item_with_id<T>(key: &str, item: &Item<T>, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup
where
    T: RenderNestedWithId,
{
    match item {
        Item::Normal(value) => html! {
            div.content-item data-key=(key) {
                (value.render_with_id(key, highlighter, toc_entries))
            }
        },
        Item::List(items) => html! {
            div.content-list data-key=(key) {
                @for value in items {
                    div.content-list-item {
                        (value.render_with_id(key, highlighter, toc_entries))
                    }
                }
            }
        },
        Item::Toc(_) => {
            if toc_entries.is_empty() {
                html! {}
            } else {
                render_toc(toc_entries)
            }
        }
    }
}

/// Trait for rendering nested content with ID support
trait RenderNestedWithId {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup;
}

impl RenderNestedWithId for Text {
    fn render_with_id(&self, _id: &str, highlighter: &CodeHighlighter, _toc_entries: &[TocEntry]) -> Markup {
        render_text(self, highlighter)
    }
}

impl<T: RenderNestedWithId> RenderNestedWithId for TextOrNested<T> {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup {
        match self {
            TextOrNested::Text(text) => render_text(text, highlighter),
            TextOrNested::Nested(nested) => nested.render_with_id(id, highlighter, toc_entries),
        }
    }
}

impl RenderNestedWithId for Level2 {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup {
        render_section_with_id(id, &self.header, &self.sections, "h2", highlighter, toc_entries)
    }
}

impl RenderNestedWithId for Level3 {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup {
        render_section_with_id(id, &self.header, &self.sections, "h3", highlighter, toc_entries)
    }
}

impl RenderNestedWithId for Level4 {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup {
        render_section_with_id(id, &self.header, &self.sections, "h4", highlighter, toc_entries)
    }
}

impl RenderNestedWithId for Level5 {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup {
        render_section_with_id(id, &self.header, &self.sections, "h5", highlighter, toc_entries)
    }
}

impl RenderNestedWithId for Level6 {
    fn render_with_id(&self, id: &str, highlighter: &CodeHighlighter, toc_entries: &[TocEntry]) -> Markup {
        render_section_with_id(id, &self.header, &self.sections, "h6", highlighter, toc_entries)
    }
}

fn render_section_with_id<T: RenderNestedWithId>(
    id: &str,
    header: &Text,
    sections: &IndexMap<String, Item<T>>,
    level: &str,
    highlighter: &CodeHighlighter,
    toc_entries: &[TocEntry],
) -> Markup {
    let section_class = format!("article-section article-section-{}", level);
    html! {
        section class=(section_class) {
            @match level {
                "h2" => h2.section-header id=(id) { (render_text(header, highlighter)) },
                "h3" => h3.section-header id=(id) { (render_text(header, highlighter)) },
                "h4" => h4.section-header id=(id) { (render_text(header, highlighter)) },
                "h5" => h5.section-header id=(id) { (render_text(header, highlighter)) },
                "h6" => h6.section-header id=(id) { (render_text(header, highlighter)) },
                _ => h2.section-header id=(id) { (render_text(header, highlighter)) },
            }
            @for (key, item) in sections {
                (render_item_with_id(key, item, highlighter, toc_entries))
            }
        }
    }
}

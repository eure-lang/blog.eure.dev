use std::collections::HashSet;

use eure::value::Text;
use indexmap::IndexMap;
use maud::{Markup, PreEscaped, html};

use crate::article::{
    AlertType, Article, Item, Level2, Level3, Level4, Level5, Level6, MarkOptions, TextOrNested,
    TocEntry,
};
use crate::render::{CodeHighlighter, render_text};
use crate::templates::base::{base_layout, OgpMeta, BASE_URL};

// GitHub Octicons SVG icons for alerts
const NOTE_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"/></svg>"#;
const TIP_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 0 1-1.484.211c-.04-.282-.163-.547-.37-.847a8.456 8.456 0 0 0-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.751.751 0 0 1-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75ZM5.75 12h4.5a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1 0-1.5ZM6 15.25a.75.75 0 0 1 .75-.75h2.5a.75.75 0 0 1 0 1.5h-2.5a.75.75 0 0 1-.75-.75Z"/></svg>"#;
const IMPORTANT_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13H8.06l-2.573 2.573A1.458 1.458 0 0 1 3 14.543V13H1.75A1.75 1.75 0 0 1 0 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h6.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Zm7 2.25v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"/></svg>"#;
const WARNING_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"/></svg>"#;
const CAUTION_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M4.47.22A.749.749 0 0 1 5 0h6c.199 0 .389.079.53.22l4.25 4.25c.141.14.22.331.22.53v6a.749.749 0 0 1-.22.53l-4.25 4.25A.749.749 0 0 1 11 16H5a.749.749 0 0 1-.53-.22L.22 11.53A.749.749 0 0 1 0 11V5c0-.199.079-.389.22-.53Zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5ZM8 4a.75.75 0 0 1 .75.75v3.5a.75.75 0 0 1-1.5 0v-3.5A.75.75 0 0 1 8 4Zm0 8a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"/></svg>"#;

/// Render text content with optional mark options (e.g., alert boxes)
fn render_text_with_mark(text: &Text, mark: &MarkOptions, highlighter: &CodeHighlighter) -> Markup {
    if mark.dangerously_inner_html {
        assert!(
            text.language.is_other("html"),
            "HTML content must be marked as HTML but got {:?}",
            text.language
        );
        return html! { div.markdown-content { (PreEscaped(text.as_str())) } };
    }

    let content = render_text(text, highlighter);

    if let Some(alert_type) = &mark.alert {
        let (class_suffix, icon, title) = match alert_type {
            AlertType::Note => ("note", NOTE_ICON, "Note"),
            AlertType::Tip => ("tip", TIP_ICON, "Tip"),
            AlertType::Important => ("important", IMPORTANT_ICON, "Important"),
            AlertType::Warning => ("warning", WARNING_ICON, "Warning"),
            AlertType::Caution => ("caution", CAUTION_ICON, "Caution"),
        };

        html! {
            div.alert class=(format!("alert-{}", class_suffix)) {
                div.alert-title {
                    span.alert-icon { (PreEscaped(icon)) }
                    span.alert-title-text { (title) }
                }
                div.alert-content { (content) }
            }
        }
    } else {
        content
    }
}

const GITHUB_REPO: &str = "eure-lang/blog.eure.dev";

pub fn render_article_page(
    article: &Article,
    slug: &str,
    commit_hash: Option<&str>,
    highlighter: &CodeHighlighter,
) -> Result<Markup, String> {
    // Collect TOC entries and validate unique IDs
    let mut seen_ids = HashSet::new();
    let toc_entries = collect_toc_entries(&article.sections, &mut seen_ids)?;

    let github_url = commit_hash.map(|hash| {
        format!(
            "https://github.com/{}/blob/{}/articles/{}.eure",
            GITHUB_REPO, hash, slug
        )
    });

    let content = html! {
        article.article {
            header.article-header {
                h1.article-title { (render_text(&article.header, highlighter)) }
                div.article-meta {
                    @if let Some(date) = &article.frontmatter.date {
                        time.article-date { (date.as_str()) }
                    }
                    div.article-links {
                        a.article-source-link href=(format!("/source/{}.html", slug)) { "Source" }
                        @if let Some(url) = &github_url {
                            a.article-github-link href=(url) target="_blank" rel="noopener noreferrer" { "GitHub" }
                        }
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

    let url = format!("{}/articles/{}.html", BASE_URL, slug);
    let ogp = OgpMeta {
        title: article.frontmatter.title.as_str(),
        description: article.frontmatter.description.as_str(),
        url: &url,
        og_type: "article",
    };
    Ok(base_layout(article.frontmatter.title.as_str(), content, &ogp))
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
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String>;
}

impl CollectToc for Text {
    fn collect_toc_entry(
        &self,
        _id: &str,
        _seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
        // Text nodes don't generate TOC entries
        Ok(None)
    }
}

impl<T: CollectToc> CollectToc for TextOrNested<T> {
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
        match self {
            TextOrNested::Text { .. } => Ok(None),
            TextOrNested::Nested(nested) => nested.collect_toc_entry(id, seen_ids),
        }
    }
}

impl CollectToc for Level2 {
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
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
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
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
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
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
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
        if !seen_ids.insert(id.to_string()) {
            return Err(format!("Duplicate section ID: {}", id));
        }
        collect_toc_entries(&self.sections, seen_ids)?;
        Ok(None)
    }
}

impl CollectToc for Level6 {
    fn collect_toc_entry(
        &self,
        id: &str,
        seen_ids: &mut HashSet<String>,
    ) -> Result<Option<TocEntry>, String> {
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
fn render_item_with_id<T>(
    key: &str,
    item: &Item<T>,
    highlighter: &CodeHighlighter,
    toc_entries: &[TocEntry],
) -> Markup
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
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup;
}

impl RenderNestedWithId for Text {
    fn render_with_id(
        &self,
        _id: &str,
        highlighter: &CodeHighlighter,
        _toc_entries: &[TocEntry],
    ) -> Markup {
        render_text(self, highlighter)
    }
}

impl<T: RenderNestedWithId> RenderNestedWithId for TextOrNested<T> {
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup {
        match self {
            TextOrNested::Text { text, mark } => render_text_with_mark(text, mark, highlighter),
            TextOrNested::Nested(nested) => nested.render_with_id(id, highlighter, toc_entries),
        }
    }
}

impl RenderNestedWithId for Level2 {
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup {
        render_section_with_id(
            id,
            &self.header,
            &self.sections,
            "h2",
            highlighter,
            toc_entries,
        )
    }
}

impl RenderNestedWithId for Level3 {
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup {
        render_section_with_id(
            id,
            &self.header,
            &self.sections,
            "h3",
            highlighter,
            toc_entries,
        )
    }
}

impl RenderNestedWithId for Level4 {
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup {
        render_section_with_id(
            id,
            &self.header,
            &self.sections,
            "h4",
            highlighter,
            toc_entries,
        )
    }
}

impl RenderNestedWithId for Level5 {
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup {
        render_section_with_id(
            id,
            &self.header,
            &self.sections,
            "h5",
            highlighter,
            toc_entries,
        )
    }
}

impl RenderNestedWithId for Level6 {
    fn render_with_id(
        &self,
        id: &str,
        highlighter: &CodeHighlighter,
        toc_entries: &[TocEntry],
    ) -> Markup {
        render_section_with_id(
            id,
            &self.header,
            &self.sections,
            "h6",
            highlighter,
            toc_entries,
        )
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

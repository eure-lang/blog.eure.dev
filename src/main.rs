mod article;
mod render;
mod templates;

use std::fs;
use std::process::Command;

use article::Article;
use render::{CodeHighlighter, eure_highlight::generate_eure_css};
use templates::{index::ArticleEntry, render_article_page, render_index_page, render_source_page};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create directories
    fs::create_dir_all("dist/articles")?;
    fs::create_dir_all("dist/source")?;
    fs::create_dir_all("dist/styles")?;

    // 2. Copy favicon assets
    copy_favicon_assets()?;

    // 3. Get git commit hash for GitHub links
    let commit_hash = get_git_commit_hash();
    if let Some(ref hash) = commit_hash {
        println!("Git commit: {}", hash);
    }

    // 4. Initialize highlighter
    let highlighter = CodeHighlighter::new()?;

    // 5. Generate CSS
    let syntax_css = highlighter.generate_css()?;
    fs::write("dist/styles/syntax.css", syntax_css)?;
    fs::write("dist/styles/eure-syntax.css", generate_eure_css())?;
    fs::write("dist/styles/main.css", generate_main_css())?;

    // 6. Read and parse articles (slug, source_content, article)
    let mut articles: Vec<(String, String, Article)> = Vec::new();
    let mut parse_errors: Vec<(std::path::PathBuf, String)> = Vec::new();
    for entry in fs::read_dir("articles")? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "eure") {
            let content = fs::read_to_string(&path)?;
            match parse_article(&content) {
                Ok(article) => {
                    // Skip drafts
                    if article.frontmatter.draft {
                        println!("Skipping draft: {:?}", path);
                        continue;
                    }
                    let slug = path.file_stem().unwrap().to_string_lossy().to_string();
                    articles.push((slug, content, article));
                }
                Err(e) => {
                    parse_errors.push((path.clone(), e));
                }
            }
        }
    }

    // Fail build if there were parse errors
    if !parse_errors.is_empty() {
        eprintln!("\nBuild failed with {} parse error(s):", parse_errors.len());
        for (path, err) in &parse_errors {
            eprintln!("  {:?}: {}", path, err);
        }
        return Err("Article parse errors".into());
    }

    // Sort by slug (descending for newest first)
    articles.sort_by(|a, b| b.0.cmp(&a.0));

    // 7. Generate article pages and source pages
    for (slug, source_content, article) in &articles {
        // Generate article page
        match render_article_page(article, slug, commit_hash.as_deref(), &highlighter) {
            Ok(html) => {
                let path = format!("dist/articles/{}.html", slug);
                fs::write(&path, html.into_string())?;
                println!("Generated: {}", path);
            }
            Err(e) => {
                eprintln!("Error rendering {}: {}", slug, e);
                return Err(e.into());
            }
        }

        // Generate source page
        let source_html = render_source_page(
            slug,
            article.frontmatter.title.as_str(),
            source_content,
            commit_hash.as_deref(),
            &highlighter,
        );
        let source_path = format!("dist/source/{}.html", slug);
        fs::write(&source_path, source_html.into_string())?;
        println!("Generated: {}", source_path);
    }

    // 8. Generate index page
    let entries: Vec<ArticleEntry> = articles
        .iter()
        .map(|(slug, _, article)| ArticleEntry {
            slug: slug.as_str(),
            article,
        })
        .collect();
    let index_html = render_index_page(&entries);
    fs::write("dist/index.html", index_html.into_string())?;
    println!("Generated: dist/index.html");

    println!("\nBuild complete! {} articles generated.", articles.len());
    Ok(())
}

fn get_git_commit_hash() -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
}

fn parse_article(input: &str) -> Result<Article, String> {
    let cst = eure::parol::parse(input).map_err(|e| format!("Parse error: {:?}", e))?;
    let doc = eure::document::cst_to_document(input, &cst)
        .map_err(|e| format!("Document error: {:?}", e))?;
    let article: Article = doc
        .parse(doc.get_root_id())
        .map_err(|e| format!("Article parse error: {:?}", e))?;
    Ok(article)
}

fn copy_favicon_assets() -> Result<(), Box<dyn std::error::Error>> {
    let assets_dir = "assets";
    let favicon_files = [
        "favicon.ico",
        "favicon-16x16.png",
        "favicon-32x32.png",
        "apple-touch-icon.png",
        "android-chrome-192x192.png",
        "android-chrome-512x512.png",
        "ogp.png",
    ];

    for file in &favicon_files {
        let src = format!("{}/{}", assets_dir, file);
        let dst = format!("dist/{}", file);
        if std::path::Path::new(&src).exists() {
            fs::copy(&src, &dst)?;
            println!("Copied: {}", dst);
        } else {
            eprintln!("Warning: {} not found", src);
        }
    }

    // Generate site.webmanifest
    let manifest = r##"{
    "name": "Eure Blog",
    "short_name": "Eure Blog",
    "icons": [
        {
            "src": "/android-chrome-192x192.png",
            "sizes": "192x192",
            "type": "image/png"
        },
        {
            "src": "/android-chrome-512x512.png",
            "sizes": "512x512",
            "type": "image/png"
        }
    ],
    "theme_color": "#1e1e2e",
    "background_color": "#1e1e2e",
    "display": "standalone"
}"##;
    fs::write("dist/site.webmanifest", manifest)?;
    println!("Generated: dist/site.webmanifest");

    Ok(())
}

fn generate_main_css() -> String {
    r#"/* Main Styles - Catppuccin Mocha Theme */
:root {
    --ctp-base: #1e1e2e;
    --ctp-mantle: #181825;
    --ctp-crust: #11111b;
    --ctp-surface0: #313244;
    --ctp-surface1: #45475a;
    --ctp-surface2: #585b70;
    --ctp-overlay0: #6c7086;
    --ctp-overlay1: #7f849c;
    --ctp-overlay2: #9399b2;
    --ctp-text: #cdd6f4;
    --ctp-subtext0: #a6adc8;
    --ctp-subtext1: #bac2de;
    --ctp-lavender: #b4befe;
    --ctp-blue: #89b4fa;
    --ctp-sapphire: #74c7ec;
    --ctp-sky: #89dceb;
    --ctp-teal: #94e2d5;
    --ctp-green: #a6e3a1;
    --ctp-yellow: #f9e2af;
    --ctp-peach: #fab387;
    --ctp-maroon: #eba0ac;
    --ctp-red: #f38ba8;
    --ctp-mauve: #cba6f7;
    --ctp-pink: #f5c2e7;
    --ctp-flamingo: #f2cdcd;
    --ctp-rosewater: #f5e0dc;
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html {
    font-size: 16px;
    line-height: 1.6;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background-color: var(--ctp-base);
    color: var(--ctp-text);
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

a {
    color: var(--ctp-blue);
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

/* Site Header */
.site-header {
    background-color: var(--ctp-mantle);
    padding: 1rem 2rem;
    border-bottom: 1px solid var(--ctp-surface0);
}

.site-nav {
    max-width: 900px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.header-left {
    display: flex;
    align-items: flex-end;
    gap: 1rem;
}

.divider {
    width: 1px;
    height: 2rem;
    background-color: currentColor;
    opacity: 0.3;
    transform: translateY(0.25rem);
}

.eure-logo {
    display: flex;
    align-items: center;
    transition: opacity 0.2s;
}

.eure-logo:hover {
    opacity: 0.8;
}

.eure-logo img {
    height: 32px;
}

.header-nav-links {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.header-nav-links a {
    color: var(--ctp-text);
    transition: opacity 0.2s;
}

.header-nav-links a:hover {
    opacity: 0.8;
    text-decoration: none;
}

.github-link {
    display: flex;
    align-items: center;
}

.site-title {
    font-size: 1.7rem;
    font-weight: bold;
    line-height: 1;
    color: var(--ctp-mauve);
}

.site-title:hover {
    color: var(--ctp-pink);
    text-decoration: none;
}

/* Site Main */
.site-main {
    flex: 1;
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
    width: 100%;
}

/* Site Footer */
.site-footer {
    background-color: var(--ctp-mantle);
    padding: 1rem 2rem;
    text-align: center;
    border-top: 1px solid var(--ctp-surface0);
    color: var(--ctp-overlay1);
}

/* Index Page */
.index-page {
    padding: 1rem 0;
}

.page-title {
    font-size: 2rem;
    margin-bottom: 2rem;
    color: var(--ctp-mauve);
}

.article-list {
    list-style: none;
}

.article-list-item {
    padding: 1rem 0;
    border-bottom: 1px solid var(--ctp-surface0);
}

.article-list-item:last-child {
    border-bottom: none;
}

.article-link {
    display: block;
    margin-bottom: 0.5rem;
}

.article-link .article-title {
    font-size: 1.25rem;
    color: var(--ctp-text);
}

.article-link:hover .article-title {
    color: var(--ctp-blue);
}

.article-date {
    color: var(--ctp-overlay1);
    font-size: 0.875rem;
}

.article-tags {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin-top: 0.5rem;
}

.article-tag {
    background-color: var(--ctp-surface0);
    color: var(--ctp-subtext1);
    padding: 0.125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
}

/* Article Page */
.article {
    padding: 1rem 0;
}

.article-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--ctp-surface0);
}

.article-title {
    font-size: 2rem;
    color: var(--ctp-text);
    margin-bottom: 0.5rem;
    line-height: 1.3;
}

.article-meta {
    color: var(--ctp-overlay1);
}

.article-content {
    line-height: 1.8;
}

/* Table of Contents */
.article-toc {
    margin-bottom: 2rem;
    border: 1px solid var(--ctp-surface1);
    border-radius: 8px;
}

.article-toc summary {
    cursor: pointer;
    padding: 0.75rem 1rem;
    font-weight: 600;
    color: var(--ctp-text);
}

.article-toc summary:hover {
    background-color: var(--ctp-surface0);
}

.article-toc nav {
    padding: 0 1rem 1rem 1rem;
}

.article-toc ul {
    list-style: none;
    padding-left: 1.5rem;
    margin: 0;
}

.article-toc > nav > ul {
    padding-left: 0;
}

.article-toc li {
    margin: 0.25rem 0;
}

.article-toc a {
    color: var(--ctp-blue);
    text-decoration: none;
}

.article-toc a:hover {
    text-decoration: underline;
}

.article-section {
    margin: 2rem 0;
}

.section-header {
    color: var(--ctp-mauve);
    margin-bottom: 1rem;
}

h2.section-header { font-size: 1.5rem; }
h3.section-header { font-size: 1.25rem; }
h4.section-header { font-size: 1.125rem; }
h5.section-header { font-size: 1rem; }
h6.section-header { font-size: 0.875rem; }

.content-item,
.content-list-item {
    margin: 1rem 0;
}

/* Text Styles */
.text-plain {
    display: block;
}

.code-inline {
    background-color: var(--ctp-surface0);
    color: var(--ctp-peach);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.875em;
}

.code-block {
    background-color: var(--ctp-mantle);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.875rem;
    line-height: 1.5;
    margin: 1rem 0;
    border: 1px solid var(--ctp-surface1);
    box-shadow: 0 4px 20px rgba(69, 71, 90, 0.8);
}

/* Markdown Content */
.markdown-content {
    line-height: 1.8;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
    color: var(--ctp-mauve);
    margin: 1.5rem 0 0.75rem;
}

.markdown-content p {
    margin: 1rem 0;
}

.markdown-content ul,
.markdown-content ol {
    margin: 1rem 0;
    padding-left: 2rem;
}

.markdown-content li {
    margin: 0.5rem 0;
}

.markdown-content code {
    background-color: var(--ctp-surface0);
    color: var(--ctp-peach);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.875em;
}

.markdown-content pre {
    background-color: var(--ctp-mantle);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    margin: 1rem 0;
    border: 1px solid var(--ctp-surface1);
    box-shadow: 0 4px 20px rgba(69, 71, 90, 0.8);
}

.markdown-content pre code {
    background: none;
    padding: 0;
}

.markdown-content blockquote {
    border-left: 3px solid var(--ctp-mauve);
    padding-left: 1rem;
    margin: 1rem 0;
    color: var(--ctp-subtext1);
}

.markdown-content a {
    color: var(--ctp-blue);
}

.markdown-content strong {
    color: var(--ctp-text);
    font-weight: 600;
}

.markdown-content table {
    border-collapse: collapse;
    width: 100%;
    margin: 1rem 0;
}

.markdown-content th,
.markdown-content td {
    border: 1px solid var(--ctp-surface1);
    padding: 0.5rem;
    text-align: left;
}

.markdown-content th {
    background-color: var(--ctp-surface0);
}

/* GitHub-style Alerts */
.alert {
    padding: 0.5rem 1rem;
    margin: 1rem 0;
    border-radius: 6px;
    border-left: 4px solid;
    background-color: var(--ctp-surface0);
}

.alert-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
}

.alert-icon {
    display: flex;
    align-items: center;
}

.alert-icon svg {
    width: 16px;
    height: 16px;
}

.alert-content {
    margin-left: 0;
}

.alert-content > :first-child {
    margin-top: 0;
}

.alert-content > :last-child {
    margin-bottom: 0;
}

/* Alert type colors (Catppuccin Mocha) */
.alert-note {
    border-color: var(--ctp-blue);
}

.alert-note .alert-title {
    color: var(--ctp-blue);
}

.alert-tip {
    border-color: var(--ctp-green);
}

.alert-tip .alert-title {
    color: var(--ctp-green);
}

.alert-important {
    border-color: var(--ctp-mauve);
}

.alert-important .alert-title {
    color: var(--ctp-mauve);
}

.alert-warning {
    border-color: var(--ctp-yellow);
}

.alert-warning .alert-title {
    color: var(--ctp-yellow);
}

.alert-caution {
    border-color: var(--ctp-red);
}

.alert-caution .alert-title {
    color: var(--ctp-red);
}

/* Giallo Code Blocks */
.giallo {
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.9rem;
    line-height: 1.5;
    margin: 1rem 0;
    border: 1px solid var(--ctp-surface1);
    box-shadow: 0 4px 20px rgba(69, 71, 90, 0.8);
}

/* Language Badge for Code Blocks */
pre[data-language] {
    position: relative;
    padding-top: 2rem;
}

pre[data-language]::before {
    content: attr(data-language);
    position: absolute;
    top: 0;
    left: 0;
    background-color: var(--ctp-surface1);
    color: var(--ctp-subtext0);
    padding: 0.125rem 0.5rem;
    font-size: 0.75rem;
    font-family: sans-serif;
    border-radius: 0.5rem 0 0.25rem 0;
    user-select: none;
}

/* Article Links (Source, GitHub) */
.article-links {
    display: inline-flex;
    gap: 1rem;
    margin-left: 1rem;
}

.article-source-link,
.article-github-link {
    color: var(--ctp-blue);
    font-size: 0.875rem;
}

.article-source-link:hover,
.article-github-link:hover {
    text-decoration: underline;
}

/* Source Page */
.source-view {
    padding: 1rem 0;
}

.source-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--ctp-surface0);
}

.source-title {
    font-size: 1.5rem;
    color: var(--ctp-text);
    margin-bottom: 0.75rem;
}

.source-actions {
    display: flex;
    gap: 1.5rem;
    align-items: center;
}

.source-back-link {
    color: var(--ctp-blue);
}

.source-github-link {
    color: var(--ctp-blue);
}

.source-content {
    margin-top: 1rem;
}

/* Responsive */
@media (max-width: 768px) {
    .site-main {
        padding: 1rem;
    }

    .article-title {
        font-size: 1.5rem;
    }
}
"#
    .to_string()
}

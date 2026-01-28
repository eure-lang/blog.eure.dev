use eure::{
    parol::{ParseResult, parse_tolerant},
    query::{SemanticToken, SemanticTokenModifier, SemanticTokenType, semantic_tokens},
};
use maud::{Markup, PreEscaped, html};

use crate::render::code_highlight::CodeHighlighter;

pub fn render_eure_highlighted(content: &str, highlighter: Option<&CodeHighlighter>) -> Markup {
    render_eure_highlighted_inner(content, false, highlighter)
}

pub fn render_eure_highlighted_with_line_numbers(
    content: &str,
    highlighter: Option<&CodeHighlighter>,
) -> Markup {
    render_eure_highlighted_inner(content, true, highlighter)
}

fn render_eure_highlighted_inner(
    content: &str,
    with_line_numbers: bool,
    highlighter: Option<&CodeHighlighter>,
) -> Markup {
    let cst = match parse_tolerant(content) {
        ParseResult::Ok(cst) => cst,
        ParseResult::ErrWithCst { cst, .. } => cst,
    };
    let tokens = semantic_tokens(content, &cst);
    let code_blocks = find_code_block_regions(content);

    if with_line_numbers {
        html! {
            pre.eure-source.eure-source-with-lines {
                code {
                    (render_tokens_by_line(content, &tokens, &code_blocks, highlighter))
                }
            }
        }
    } else {
        html! {
            pre.code-block {
                code {
                    (render_tokens(content, &tokens, &code_blocks, highlighter))
                }
            }
        }
    }
}

/// Represents a code block region in the source
struct CodeBlockRegion {
    /// Start of content (after opening ``` and newline)
    content_start: usize,
    /// End of content (before closing ```)
    content_end: usize,
    /// The language tag (e.g., "markdown", "rust")
    language: String,
}

/// Find all code block regions in the content
fn find_code_block_regions(content: &str) -> Vec<CodeBlockRegion> {
    let mut regions = Vec::new();
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        // Look for ``` (3-6 backticks)
        if i + 3 <= len && &bytes[i..i + 3] == b"```" {
            let start = i;
            let mut backtick_count = 3;

            // Count additional backticks (up to 6 total)
            while i + backtick_count < len
                && bytes[i + backtick_count] == b'`'
                && backtick_count < 6
            {
                backtick_count += 1;
            }

            // Extract language tag (until newline or whitespace)
            let after_backticks = i + backtick_count;
            let mut lang_end = after_backticks;
            while lang_end < len && bytes[lang_end] != b'\n' && !bytes[lang_end].is_ascii_whitespace()
            {
                lang_end += 1;
            }
            let language = content[after_backticks..lang_end].to_string();

            // Find the newline after opening
            let mut newline_pos = lang_end;
            while newline_pos < len && bytes[newline_pos] != b'\n' {
                newline_pos += 1;
            }

            if newline_pos < len {
                let content_start = newline_pos + 1;

                // Find closing backticks (same count)
                let closing_pattern = &content[start..start + backtick_count];
                if let Some(rel_close) = content[content_start..].find(closing_pattern) {
                    let content_end = content_start + rel_close;

                    regions.push(CodeBlockRegion {
                        content_start,
                        content_end,
                        language,
                    });

                    // Skip past the closing backticks
                    i = content_end + backtick_count;
                    continue;
                }
            }
        }
        i += 1;
    }

    regions
}

/// Check if a range overlaps with a code block content region
fn find_code_block_for_range(
    gap_start: usize,
    gap_end: usize,
    regions: &[CodeBlockRegion],
) -> Option<&CodeBlockRegion> {
    regions
        .iter()
        .find(|r| gap_start < r.content_end && gap_end > r.content_start)
}

fn render_tokens_to_string(
    content: &str,
    tokens: &[SemanticToken],
    code_blocks: &[CodeBlockRegion],
    highlighter: Option<&CodeHighlighter>,
) -> String {
    let mut html_output = String::new();
    let mut last_end: usize = 0;

    for token in tokens {
        let start = token.start as usize;
        let end = start + token.length as usize;

        // Handle gap before this token
        if start > last_end {
            let gap_text = &content[last_end..start];

            // Check if this gap overlaps with a code block content region
            if let Some(region) = find_code_block_for_range(last_end, start, code_blocks) {
                if !region.language.is_empty() {
                    if let Some(h) = highlighter {
                        // Calculate the overlap between gap and code block content
                        let content_in_gap_start = region.content_start.max(last_end);
                        let content_in_gap_end = region.content_end.min(start);

                        // Output any part of the gap before the code block content
                        if last_end < content_in_gap_start {
                            html_output.push_str(&html_escape(&content[last_end..content_in_gap_start]));
                        }

                        // Highlight the code block content part
                        let code_content = &content[content_in_gap_start..content_in_gap_end];
                        if region.language == "eure" {
                            // Use eure semantic token highlighting (without wrapper)
                            html_output.push_str(&render_eure_tokens_only(code_content));
                        } else {
                            for (i, line) in code_content.split('\n').enumerate() {
                                if i > 0 {
                                    html_output.push('\n');
                                }
                                if let Some(highlighted) = h.highlight_line(line, &region.language) {
                                    html_output.push_str(&highlighted.into_string());
                                } else {
                                    html_output.push_str(&html_escape(line));
                                }
                            }
                        }

                        // Output any part of the gap after the code block content
                        if content_in_gap_end < start {
                            html_output.push_str(&html_escape(&content[content_in_gap_end..start]));
                        }
                    } else {
                        html_output.push_str(&html_escape(gap_text));
                    }
                } else {
                    html_output.push_str(&html_escape(gap_text));
                }
            } else {
                html_output.push_str(&html_escape(gap_text));
            }
        }

        let token_text = &content[start..end];
        let text = html_escape(token_text);
        let classes = build_classes(token);
        html_output.push_str(&format!("<span class=\"{}\">{}</span>", classes, text));

        last_end = end;
    }

    if last_end < content.len() {
        html_output.push_str(&html_escape(&content[last_end..]));
    }

    html_output
}

fn render_tokens(
    content: &str,
    tokens: &[SemanticToken],
    code_blocks: &[CodeBlockRegion],
    highlighter: Option<&CodeHighlighter>,
) -> Markup {
    html! { (PreEscaped(render_tokens_to_string(content, tokens, code_blocks, highlighter))) }
}

fn render_tokens_by_line(
    content: &str,
    tokens: &[SemanticToken],
    code_blocks: &[CodeBlockRegion],
    highlighter: Option<&CodeHighlighter>,
) -> Markup {
    let html_output = render_tokens_to_string(content, tokens, code_blocks, highlighter);
    let result: String = html_output
        .split('\n')
        .map(|line| format!("<span class=\"line\">{}</span>", line))
        .collect();
    html! { (PreEscaped(result)) }
}

/// Render eure tokens only (without pre/code wrapper) for nested code blocks
fn render_eure_tokens_only(content: &str) -> String {
    let cst = match parse_tolerant(content) {
        ParseResult::Ok(cst) => cst,
        ParseResult::ErrWithCst { cst, .. } => cst,
    };
    let tokens = semantic_tokens(content, &cst);

    let mut html_output = String::new();
    let mut last_end: usize = 0;

    for token in &tokens {
        let start = token.start as usize;
        let end = start + token.length as usize;

        if start > last_end {
            html_output.push_str(&html_escape(&content[last_end..start]));
        }

        let token_text = &content[start..end];
        let text = html_escape(token_text);
        let classes = build_classes(token);
        html_output.push_str(&format!("<span class=\"{}\">{}</span>", classes, text));

        last_end = end;
    }

    if last_end < content.len() {
        html_output.push_str(&html_escape(&content[last_end..]));
    }

    html_output
}

fn build_classes(token: &SemanticToken) -> String {
    let mut classes = vec![token_type_class(token.token_type)];

    if token.modifiers & (SemanticTokenModifier::Declaration as u32) != 0 {
        classes.push("eure-mod-declaration");
    }
    if token.modifiers & (SemanticTokenModifier::Definition as u32) != 0 {
        classes.push("eure-mod-definition");
    }
    if token.modifiers & (SemanticTokenModifier::SectionHeader as u32) != 0 {
        classes.push("eure-mod-section-header");
    }

    classes.join(" ")
}

fn token_type_class(t: SemanticTokenType) -> &'static str {
    match t {
        SemanticTokenType::Keyword => "eure-keyword",
        SemanticTokenType::Number => "eure-number",
        SemanticTokenType::String => "eure-string",
        SemanticTokenType::Comment => "eure-comment",
        SemanticTokenType::Operator => "eure-operator",
        SemanticTokenType::Property => "eure-property",
        SemanticTokenType::Punctuation => "eure-punctuation",
        SemanticTokenType::Macro => "eure-macro",
        SemanticTokenType::Decorator => "eure-decorator",
        SemanticTokenType::SectionMarker => "eure-section-marker",
        SemanticTokenType::ExtensionMarker => "eure-extension-marker",
        SemanticTokenType::ExtensionIdent => "eure-extension-ident",
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Generate CSS for eure syntax highlighting using Catppuccin Mocha colors
pub fn generate_eure_css() -> String {
    r#"/* Eure Syntax Highlighting - Catppuccin Mocha */
.eure-source {
    background-color: #1e1e2e;
    color: #cdd6f4;
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.9rem;
    line-height: 1.5;
}

.eure-keyword { color: #cba6f7; } /* mauve */
.eure-number { color: #fab387; } /* peach */
.eure-string { color: #a6e3a1; } /* green */
.eure-comment { color: #6c7086; font-style: italic; } /* overlay0 */
.eure-operator { color: #89dceb; } /* sky */
.eure-property { color: #89b4fa; } /* blue */
.eure-punctuation { color: #9399b2; } /* overlay2 */
.eure-macro { color: #f38ba8; } /* red */
.eure-decorator { color: #f9e2af; } /* yellow */
.eure-section-marker { color: #f5c2e7; font-weight: bold; } /* pink */
.eure-extension-marker { color: #94e2d5; } /* teal */
.eure-extension-ident { color: #94e2d5; } /* teal */

/* Modifiers */
.eure-mod-declaration { font-weight: 600; }
.eure-mod-definition { font-weight: bold; }
.eure-mod-section-header { text-decoration: underline; }

/* Line numbers and wrap (source page only) */
.eure-source-with-lines {
    counter-reset: line;
    white-space: pre-wrap;
    word-break: break-all;
    padding: 0;
    margin: 0;
    overflow: visible;
}

.eure-source-with-lines code {
    display: block;
    padding: 0;
    margin: 0;
}

.eure-source-with-lines .line {
    display: block;
    counter-increment: line;
    position: relative;
    min-height: 1.5em;
}

.eure-source-with-lines .line::before {
    content: counter(line);
    position: absolute;
    right: 100%;
    width: 3em;
    margin-right: 0.5em;
    text-align: right;
    color: #6c7086;
    user-select: none;
}

"#
    .to_string()
}

use eure::{
    parol::{ParseResult, parse_tolerant},
    query::{SemanticToken, SemanticTokenModifier, SemanticTokenType, semantic_tokens},
};
use maud::{Markup, PreEscaped, html};

pub fn render_eure_highlighted(content: &str) -> Markup {
    let cst = match parse_tolerant(content) {
        ParseResult::Ok(cst) => cst,
        ParseResult::ErrWithCst { cst, .. } => cst,
    };
    let tokens = semantic_tokens(content, &cst);

    html! {
        pre.code-block.code-block-eure {
            code {
                (render_tokens(content, &tokens))
            }
        }
    }
}

fn render_tokens(content: &str, tokens: &[SemanticToken]) -> Markup {
    let mut html_output = String::new();
    let mut last_end: usize = 0;

    for token in tokens {
        let start = token.start as usize;
        let end = start + token.length as usize;

        // Text before token
        if start > last_end {
            html_output.push_str(&html_escape(&content[last_end..start]));
        }

        // Token itself
        let text = html_escape(&content[start..end]);
        let classes = build_classes(token);
        html_output.push_str(&format!("<span class=\"{}\">{}</span>", classes, text));

        last_end = end;
    }

    // Remaining text
    if last_end < content.len() {
        html_output.push_str(&html_escape(&content[last_end..]));
    }

    html! { (PreEscaped(html_output)) }
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
.code-block-eure {
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
"#
    .to_string()
}

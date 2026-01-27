use eure::value::{Language, Text};
use maud::{html, Markup, PreEscaped};

use crate::render::{code_highlight::CodeHighlighter, eure_highlight::render_eure_highlighted};

pub fn render_text(text: &Text, highlighter: &CodeHighlighter) -> Markup {
    match &text.language {
        Language::Plaintext => html! { span.text-plain { (text.as_str()) } },
        Language::Implicit => html! { code.code-inline { (text.as_str()) } },
        Language::Other(lang) => render_with_language(text.as_str(), lang, highlighter),
    }
}

fn render_with_language(content: &str, lang: &str, highlighter: &CodeHighlighter) -> Markup {
    match lang {
        "markdown" => render_markdown(content),
        "eure" => render_eure_highlighted(content),
        other => highlighter.highlight(content, other),
    }
}

fn render_markdown(content: &str) -> Markup {
    use markdown::{to_html_with_options, CompileOptions, Options};
    let options = Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..CompileOptions::default()
        },
        ..Options::gfm()
    };
    let html_output =
        to_html_with_options(content, &options).unwrap_or_else(|_| content.to_string());
    html! { div.markdown-content { (PreEscaped(html_output)) } }
}

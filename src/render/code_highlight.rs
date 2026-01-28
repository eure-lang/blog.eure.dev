use giallo::{FontStyle, HighlightOptions, HtmlRenderer, Registry, RenderOptions, ThemeVariant};
// HtmlRenderer and RenderOptions are used in the `highlight` method for article code blocks
use maud::{html, Markup, PreEscaped};

pub struct CodeHighlighter {
    registry: Registry,
}

impl CodeHighlighter {
    pub fn new() -> Result<Self, giallo::Error> {
        let mut registry = Registry::builtin()?;
        registry.link_grammars();
        Ok(Self { registry })
    }

    pub fn generate_css(&self) -> Result<String, giallo::Error> {
        self.registry.generate_css("catppuccin-mocha", "hl-")
    }

    pub fn highlight(&self, code: &str, language: &str) -> Markup {
        let options = HighlightOptions::new(language, ThemeVariant::Single("catppuccin-mocha"));

        let html_output = match self.registry.highlight(code, &options) {
            Ok(highlighted) => {
                let renderer = HtmlRenderer {
                    css_class_prefix: Some("hl-".to_string()),
                    ..Default::default()
                };
                renderer.render(&highlighted, &RenderOptions::default())
            }
            Err(e) => {
                panic!(
                    "Failed to highlight code for language '{}': {:?}",
                    language, e
                );
            }
        };

        // Add data-language attribute to the <pre> tag
        let display_lang = format_language_name(language);
        let html_with_lang = html_output.replacen(
            "<pre class=\"giallo hl-code\">",
            &format!("<pre class=\"giallo hl-code\" data-language=\"{}\">", display_lang),
            1,
        );

        html! { (PreEscaped(html_with_lang)) }
    }

    /// Highlight a single line of code, returning just the styled spans (no wrappers).
    pub fn highlight_line(&self, line: &str, language: &str) -> Option<Markup> {
        let options = HighlightOptions::new(language, ThemeVariant::Single("catppuccin-mocha"));
        let highlighted = self.registry.highlight(line, &options).ok()?;

        Some(html! {
            @for line_tokens in &highlighted.tokens {
                @for token in line_tokens {
                    @if let ThemeVariant::Single(style) = &token.style {
                        @let color = style.foreground.as_hex();
                        @let is_bold = style.font_style.contains(FontStyle::BOLD);
                        @let is_italic = style.font_style.contains(FontStyle::ITALIC);
                        @if is_bold {
                            span style=(format!("color:{};font-weight:bold", color)) { (token.text) }
                        } @else if is_italic {
                            span style=(format!("color:{};font-style:italic", color)) { (token.text) }
                        } @else {
                            span style=(format!("color:{}", color)) { (token.text) }
                        }
                    } @else {
                        (token.text)
                    }
                }
            }
        })
    }
}

/// Format language name for display in the badge
fn format_language_name(lang: &str) -> String {
    match lang.to_lowercase().as_str() {
        // Acronyms (uppercase)
        "toml" => "TOML".to_string(),
        "yaml" => "YAML".to_string(),
        "json" => "JSON".to_string(),
        "html" => "HTML".to_string(),
        "css" => "CSS".to_string(),
        "sql" => "SQL".to_string(),
        "xml" => "XML".to_string(),
        // Names (title case)
        "rust" => "Rust".to_string(),
        "bash" | "shellscript" | "shell" | "sh" => "Bash".to_string(),
        "javascript" | "js" => "JavaScript".to_string(),
        "typescript" | "ts" => "TypeScript".to_string(),
        "python" | "py" => "Python".to_string(),
        "ruby" | "rb" => "Ruby".to_string(),
        "go" | "golang" => "Go".to_string(),
        "java" => "Java".to_string(),
        "kotlin" => "Kotlin".to_string(),
        "swift" => "Swift".to_string(),
        "c" => "C".to_string(),
        "cpp" | "c++" => "C++".to_string(),
        "csharp" | "c#" => "C#".to_string(),
        "php" => "PHP".to_string(),
        "perl" => "Perl".to_string(),
        "lua" => "Lua".to_string(),
        "r" => "R".to_string(),
        "scala" => "Scala".to_string(),
        "haskell" => "Haskell".to_string(),
        "elixir" => "Elixir".to_string(),
        "erlang" => "Erlang".to_string(),
        "clojure" => "Clojure".to_string(),
        "markdown" | "md" => "Markdown".to_string(),
        "dockerfile" => "Dockerfile".to_string(),
        "makefile" => "Makefile".to_string(),
        _ => lang.to_uppercase(),
    }
}

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

        html! { (PreEscaped(html_output)) }
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

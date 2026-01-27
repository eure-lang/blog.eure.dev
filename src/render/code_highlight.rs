use giallo::{HighlightOptions, HtmlRenderer, Registry, RenderOptions, ThemeVariant};
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
            Err(_) => {
                // Fallback: escape only
                format!(
                    "<pre class=\"code-block\"><code class=\"language-{}\">{}</code></pre>",
                    language,
                    html_escape(code)
                )
            }
        };

        html! { (PreEscaped(html_output)) }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

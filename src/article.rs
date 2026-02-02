use eure::{FromEure, value::Text};
use indexmap::IndexMap;

/// Alert type for GitHub-style alert boxes
#[derive(Debug, Clone, Copy, PartialEq, FromEure)]
pub enum AlertType {
    #[eure(rename = "NOTE")]
    Note,
    #[eure(rename = "TIP")]
    Tip,
    #[eure(rename = "IMPORTANT")]
    Important,
    #[eure(rename = "WARNING")]
    Warning,
    #[eure(rename = "CAUTION")]
    Caution,
}

/// Mark options for extending text content with visual markers
#[derive(Debug, Clone, PartialEq, FromEure, Default)]
pub struct MarkOptions {
    #[eure(default)]
    pub alert: Option<AlertType>,
    #[eure(rename = "dangerously-inner-html", default)]
    pub dangerously_inner_html: bool,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Article {
    #[eure(ext)]
    pub frontmatter: Frontmatter,
    #[eure(rename = "#")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level2>>>,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Frontmatter {
    pub title: Text,
    pub description: Text,
    #[eure(default)]
    pub date: Option<Text>,
    #[eure(default)]
    pub tags: Vec<String>,
    #[eure(default)]
    pub draft: bool,
}

/// Entry for Table of Contents
#[derive(Debug, Clone)]
pub struct TocEntry {
    pub id: String,
    pub title: String,
    #[allow(dead_code)]
    pub level: u8,
    pub children: Vec<TocEntry>,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub enum Item<T> {
    Normal(T),
    List(Vec<T>),
    Toc(Toc),
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Toc {
    #[eure(ext)]
    toc: bool,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub enum TextOrNested<T> {
    Text {
        #[eure(flatten)]
        text: Text,
        #[eure(ext, default)]
        mark: MarkOptions,
    },
    Nested(T),
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Level2 {
    #[eure(rename = "##")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level3>>>,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Level3 {
    #[eure(rename = "###")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level4>>>,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Level4 {
    #[eure(rename = "####")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level5>>>,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Level5 {
    #[eure(rename = "#####")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level6>>>,
}

#[derive(Debug, Clone, PartialEq, FromEure)]
pub struct Level6 {
    #[eure(rename = "######")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<Text>>,
}

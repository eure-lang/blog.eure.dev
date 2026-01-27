use eure::{ParseDocument, value::Text};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Article {
    #[eure(ext)]
    pub frontmatter: Frontmatter,
    #[eure(rename = "#")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level2>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Frontmatter {
    pub title: Text,
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
    pub level: u8,
    pub children: Vec<TocEntry>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub enum Item<T> {
    Normal(T),
    List(Vec<T>),
    Toc(Toc),
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Toc {
    #[eure(ext)]
    toc: bool,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub enum TextOrNested<T> {
    Text(Text),
    Nested(T),
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level2 {
    #[eure(rename = "##")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level3>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level3 {
    #[eure(rename = "###")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level4>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level4 {
    #[eure(rename = "####")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level5>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level5 {
    #[eure(rename = "#####")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<TextOrNested<Level6>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level6 {
    #[eure(rename = "######")]
    pub header: Text,
    #[eure(flatten)]
    pub sections: IndexMap<String, Item<Text>>,
}

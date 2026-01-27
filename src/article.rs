use eure::{ParseDocument, value::Text};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Article {
    #[eure(ext)]
    frontmatter: Frontmatter,
    #[eure(rename = "#")]
    header: Text,
    sections: IndexMap<String, Item<TextOrNested<Level2>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Frontmatter {
    title: Text,
    #[eure(default)]
    date: Option<Text>,
    #[eure(default)]
    tags: Vec<String>,
    #[eure(default)]
    draft: bool,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub enum Item<T> {
    Normal(T),
    List(Vec<T>),
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub enum TextOrNested<T> {
    Text(Text),
    Nested(T),
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level2 {
    #[eure(rename = "##")]
    header: Text,
    sections: IndexMap<String, Item<TextOrNested<Level3>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level3 {
    #[eure(rename = "###")]
    header: Text,
    sections: IndexMap<String, Item<TextOrNested<Level4>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level4 {
    #[eure(rename = "####")]
    header: Text,
    sections: IndexMap<String, Item<TextOrNested<Level5>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level5 {
    #[eure(rename = "#####")]
    header: Text,
    sections: IndexMap<String, Item<TextOrNested<Level6>>>,
}

#[derive(Debug, Clone, PartialEq, ParseDocument)]
pub struct Level6 {
    #[eure(rename = "######")]
    header: Text,
    sections: IndexMap<String, Item<Text>>,
}

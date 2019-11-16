use crate::Config;
use serde_derive::Serialize;

/// Context passed to the `layouts/chapter.hbs` template.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Context<'a> {
    pub chapter: ChapterInfo<'a>,
    pub html_config: &'a Config,
    pub book_config: &'a mdbook::config::BookConfig,
    pub sidebar: &'a Sidebar,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChapterInfo<'a> {
    pub content: &'a str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sidebar {}

/// Context passed to the `layouts/print.hbs` template.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PrintContext<'a> {
    pub html_config: &'a Config,
    pub book_config: &'a mdbook::config::BookConfig,
    pub sidebar: &'a Sidebar,
}

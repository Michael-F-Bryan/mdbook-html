use crate::Config;
use serde_derive::Serialize;
use std::path::Path;

/// Context passed to the `layouts/chapter.hbs` template.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Context<'a> {
    pub chapter: ChapterInfo<'a>,
    pub common: Common<'a>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Common<'a> {
    pub html_config: &'a Config,
    pub book_config: &'a mdbook::config::BookConfig,
    pub sidebar: &'a Sidebar,
    /// Where the file that we're rendering would be placed on disk.
    pub current_file: &'a Path,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ChapterInfo<'a> {
    pub content: &'a str,
    pub title: &'a str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sidebar {}

/// Context passed to the `layouts/print.hbs` template.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PrintContext<'a> {
    pub common: Common<'a>,
}

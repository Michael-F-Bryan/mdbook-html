//! An alternate HTML backend for [mdbook][md].
//!
//! [md]: https://github.com/rust-lang-nursery/mdBook

use mdbook::{errors::Error as MdError, renderer::RenderContext};
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Renderer;

impl mdbook::Renderer for Renderer {
    fn name(&self) -> &str { "html-alt" }

    fn render(&self, _ctx: &RenderContext) -> Result<(), MdError> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename = "kebab-case")]
pub struct Config {
    pub theme: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Config { Config { theme: None } }
}

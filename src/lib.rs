//! An alternate HTML backend for [mdbook][md].
//!
//! [md]: https://github.com/rust-lang-nursery/mdBook

mod renderer;
mod config;

pub use renderer::Renderer;
pub use config::Config;

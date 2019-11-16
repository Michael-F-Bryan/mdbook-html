//! An alternate HTML backend for [mdbook][md].
//!
//! [md]: https://github.com/rust-lang-nursery/mdBook

mod config;
mod renderer;
mod themes;

pub use config::Config;
pub use renderer::Renderer;
pub use themes::RegistrationError;

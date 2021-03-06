//! An alternate HTML backend for [mdbook][md].
//!
//! [md]: https://github.com/rust-lang-nursery/mdBook

mod config;
pub mod context;
pub mod helpers;
mod renderer;
mod themes;

pub use config::Config;
pub use renderer::Renderer;
pub use themes::RegistrationError;

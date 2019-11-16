use crate::Config;
use handlebars::{Handlebars, TemplateError};

/// Registers all templates and partials associated with this theme.
pub(crate) fn register(
    hbs: &mut Handlebars,
    _cfg: &Config,
) -> Result<(), RegistrationError> {
    register_default_themes(hbs)?;

    // TODO: load from the user's `theme/` directory, if it exists

    Ok(())
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RegistrationError {
    #[error("Unable to register \"{name}\"")]
    BadTemplate {
        name: String,
        #[source]
        inner: TemplateError,
    },
}

// define a couple macros to make registering items based on statically-defined
// files easier. We can't use normal functions because of `concat!()` and
// `include_str!()`.

macro_rules! theme_file {
    ($name:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/default-theme/",
            $name
        ))
    };
}

macro_rules! register {
    ($hbs:expr, $method:ident, $( $filename:expr),+) => {
        $(
            $hbs.$method($filename, theme_file!($filename))
                .map_err(|e| RegistrationError::BadTemplate {
                    name: String::from($filename),
                    inner: e,
                })?;
        )*
    };
}

macro_rules! register_layout {
    ($hbs:expr, $( $filename:expr ),+) => {
        register!($hbs, register_template_string, $( $filename ),*);
    };
}

macro_rules! register_partial {
    ($hbs:expr, $( $filename:expr, )+) => {
        register!($hbs, register_partial, $( $filename ),*);
    };
}

fn register_default_themes(
    hbs: &mut Handlebars,
) -> Result<(), RegistrationError> {
    register_layout!(hbs, "layouts/chapter.hbs", "layouts/print.hbs");

    register_partial!(
        hbs,
        "partials/after_body.hbs",
        "partials/after_head.hbs",
        "partials/content.hbs",
        "partials/footer.hbs",
        "partials/head.hbs",
        "partials/header.hbs",
        "partials/sidebar.hbs",
    );

    Ok(())
}

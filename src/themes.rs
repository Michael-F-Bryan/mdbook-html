use crate::Config;
use handlebars::{Handlebars, TemplateError};
use include_dir::{include_dir, Dir};
use std::{borrow::Cow, io, path::PathBuf};

type Assets = Vec<(PathBuf, Cow<'static, [u8]>)>;

/// Registers all templates and partials associated with this theme.
pub(crate) fn register(
    hbs: &mut Handlebars,
    _cfg: &Config,
) -> Result<(), RegistrationError> {
    register_default_themes(hbs)?;

    // TODO: load from the user's `theme/` directory, if it exists

    Ok(())
}

pub(crate) fn assets(_cfg: &Config) -> Result<Assets, io::Error> {
    static DEFAULT_STATIC_ASSETS: Dir<'_> =
        include_dir!("default-theme/static");

    let mut assets = Assets::new();

    add_files(&DEFAULT_STATIC_ASSETS, &mut assets);

    // TODO: load statics from user's `theme/static/` directory

    Ok(assets)
}

fn add_files(dir: &Dir<'static>, assets: &mut Assets) {
    for file in dir.files() {
        assets.push((file.path().to_owned(), Cow::from(file.contents())));
    }
    for subdir in dir.dirs() {
        add_files(subdir, assets);
    }
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

macro_rules! register_layout {
    ($hbs:expr, $( $filename:expr ),+) => {
        $(
            $hbs.register_template_string($filename, theme_file!($filename))
                .map_err(|e| RegistrationError::BadTemplate {
                    name: String::from($filename),
                    inner: e,
                })?;
        )*
    };
}

macro_rules! register_partial {
    ($hbs:expr, $( $filename:expr, )+) => {
        $(
            let name = std::path::Path::new($filename)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let content = theme_file!($filename);
            $hbs.register_partial(name, content)
                .map_err(|e| RegistrationError::BadTemplate {
                    name: String::from($filename),
                    inner: e,
                })?;
        )*
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

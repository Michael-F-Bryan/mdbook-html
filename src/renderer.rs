use crate::{
    context::{ChapterInfo, Common, Context, PrintContext, Sidebar},
    Config,
};
use handlebars::Handlebars;
use mdbook::{
    book::{Book, BookItem, Chapter},
    errors::{Error, ResultExt},
    renderer::RenderContext,
};
use pulldown_cmark::Parser;
use rayon::prelude::*;
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Renderer;

impl mdbook::Renderer for Renderer {
    fn name(&self) -> &str { "html-alt" }

    fn render(&self, ctx: &RenderContext) -> Result<(), Error> {
        let cfg: Config = ctx
            .config
            .get_deserialized_opt(format!("output.{}", self.name()))
            .chain_err(|| "Unable to deserialize the config")?
            .unwrap_or_default();
        log::debug!("Loaded the config: {:?}", cfg);

        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);
        crate::helpers::register(&mut hbs, ctx);
        crate::themes::register(&mut hbs, &cfg)
            .chain_err(|| "Unable to register templates")?;
        log::debug!("Set up the handlebars renderer");

        if ctx.destination.exists() {
            fs::remove_dir_all(&ctx.destination)
                .chain_err(|| "Unable to clean the output directory")?;
        }

        let plan = make_a_plan(&ctx.book, &cfg)?;
        plan.render(&hbs, &cfg, ctx)?;

        Ok(())
    }
}

fn make_a_plan<'book>(
    book: &'book Book,
    cfg: &Config,
) -> Result<Plan<'book>, Error> {
    let theme_assets = crate::themes::assets(cfg)
        .chain_err(|| "Unable to load the theme assets")?;
    let assets = theme_assets
        .into_iter()
        .map(|(filename, content)| Asset::FromTheme { filename, content })
        .collect();

    Ok(Plan {
        chapters: book
            .iter()
            .filter_map(just_chapters)
            .map(|ch| FullChapter { src: ch })
            .collect(),
        print: PrintPage { _book: book },
        sidebar: Sidebar {},
        assets,
    })
}

fn just_chapters(book_item: &BookItem) -> Option<&Chapter> {
    match book_item {
        BookItem::Chapter(ref ch) => Some(ch),
        BookItem::Separator => None,
    }
}

/// A plan for how to render a document.
struct Plan<'a> {
    chapters: Vec<FullChapter<'a>>,
    print: PrintPage<'a>,
    /// Non-text items which should be copied to the output directory without
    /// modification.
    assets: Vec<Asset>,
    sidebar: Sidebar,
}

impl<'a> Plan<'a> {
    fn render(
        &self,
        hbs: &Handlebars,
        cfg: &Config,
        ctx: &RenderContext,
    ) -> Result<(), Error> {
        self.chapters
            .par_iter()
            .try_for_each(|ch| ch.render(hbs, &self.sidebar, cfg, ctx))?;
        self.print.render(hbs, &self.sidebar, cfg, ctx)?;
        self.copy_across_static_assets(&ctx.destination)?;

        Ok(())
    }

    fn copy_across_static_assets(&self, dest_dir: &Path) -> Result<(), Error> {
        self.assets.par_iter().try_for_each(|asset| {
            asset.write_to(dest_dir).chain_err(|| {
                format!(
                    "Unable to copy across \"{}\"",
                    asset.destination_filename(dest_dir).display()
                )
            })
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Asset {
    #[allow(dead_code)]
    OnDisk {
        /// The asset's location on disk.
        src_filename: PathBuf,
        /// Where it should be written (relative to the output directory).
        destination: PathBuf,
    },
    FromTheme {
        /// The asset's path, relative to the `themes/static/` directory.
        filename: PathBuf,
        content: Cow<'static, [u8]>,
    },
}

impl Asset {
    fn destination_filename(&self, destination_dir: &Path) -> PathBuf {
        match self {
            Asset::OnDisk { destination, .. } => {
                destination_dir.join(destination)
            },
            Asset::FromTheme { filename, .. } => destination_dir.join(filename),
        }
    }

    fn write_to(&self, destination_dir: &Path) -> Result<(), Error> {
        let dest = self.destination_filename(destination_dir);
        ensure_parent_exists(&dest)?;

        match self {
            Asset::OnDisk { src_filename, .. } => {
                fs::copy(src_filename, dest)?;
                Ok(())
            },
            Asset::FromTheme { content, .. } => {
                fs::write(dest, content)?;
                Ok(())
            },
        }
    }
}

/// Normal content.
struct FullChapter<'a> {
    src: &'a Chapter,
}

fn ensure_parent_exists(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            log::debug!("Creating the \"{}\" directory", parent.display());
            fs::create_dir_all(&parent).chain_err(|| {
                format!("Unable to create \"{}\"", parent.display())
            })?;
        }
    }

    Ok(())
}

impl<'a> FullChapter<'a> {
    fn render(
        &self,
        hbs: &Handlebars,
        sidebar: &Sidebar,
        cfg: &Config,
        ctx: &RenderContext,
    ) -> Result<(), Error> {
        let mut dest = ctx.destination.join(&self.src.path);
        dest.set_extension("html");
        ensure_parent_exists(&dest)?;

        let rendered =
            render_chapter(self.src, &dest, hbs, sidebar, cfg, &ctx.config)?;

        if let Some(parent) = dest.parent() {
            if !parent.exists() {
                log::debug!("Creating the \"{}\" directory", parent.display());
                fs::create_dir_all(&parent).chain_err(|| {
                    format!("Unable to create \"{}\"", parent.display())
                })?;
            }
        }

        log::debug!(
            "Writing \"{}\" to \"{}\" ({} bytes)",
            self.src.name,
            dest.display(),
            rendered.len(),
        );

        fs::write(&dest, rendered.as_bytes())
            .chain_err(|| format!("Unable to write to \"{}\"", dest.display()))
    }
}

/// A page meant to show the entire document so it can be printed.
struct PrintPage<'a> {
    _book: &'a Book,
}

impl<'a> PrintPage<'a> {
    fn render(
        &self,
        hbs: &Handlebars,
        sidebar: &Sidebar,
        cfg: &Config,
        ctx: &RenderContext,
    ) -> Result<(), Error> {
        let dest = ctx.destination.join("print.html");
        ensure_parent_exists(&dest)?;

        let print_context = PrintContext {
            common: Common {
                html_config: cfg,
                book_config: &ctx.config.book,
                sidebar,
                current_file: &dest,
            },
        };

        if log::log_enabled!(log::Level::Trace) {
            let print_context = serde_json::to_string_pretty(&print_context)?;
            log::trace!("Rendering the print page with {}", print_context);
        }

        let rendered = hbs.render("layouts/print.hbs", &print_context)?;

        log::debug!(
            "Writing the print page to \"{}\" ({} bytes)",
            dest.display(),
            rendered.len(),
        );

        fs::write(&dest, rendered.as_bytes())
            .chain_err(|| format!("Unable to write to \"{}\"", dest.display()))
    }
}

fn render_chapter(
    chapter: &Chapter,
    dest: &Path,
    hbs: &Handlebars,
    sidebar: &Sidebar,
    cfg: &Config,
    book_cfg: &mdbook::Config,
) -> Result<String, Error> {
    let body = md_to_html(&chapter.content);
    let context = Context {
        chapter: ChapterInfo {
            content: &body,
            title: &chapter.name,
        },
        common: Common {
            html_config: cfg,
            book_config: &book_cfg.book,
            sidebar,
            current_file: dest,
        },
    };

    if log::log_enabled!(log::Level::Trace) {
        let context = serde_json::to_string_pretty(&context)?;
        log::trace!("Rendering \"{}\" with {}", chapter.name, context);
    }

    hbs.render("layouts/chapter.hbs", &context)
        .chain_err(|| format!("Unable to render \"{}\"", chapter.name))
}

fn md_to_html(src: &str) -> String {
    let mut buffer = String::with_capacity(src.len());
    pulldown_cmark::html::push_html(&mut buffer, Parser::new(src));

    buffer
}

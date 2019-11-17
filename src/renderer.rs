use crate::{
    context::{ChapterInfo, Context, PrintContext, Sidebar},
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
use std::path::{Path, PathBuf};

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
        register_default_helpers(&mut hbs);
        crate::themes::register(&mut hbs, &cfg)
            .chain_err(|| "Unable to register templates")?;
        log::debug!("Set up the handlebars renderer");

        let plan = make_a_plan(&ctx.book);
        plan.render(&hbs, &cfg, ctx)?;

        Ok(())
    }
}

fn make_a_plan(book: &Book) -> Plan<'_> {
    Plan {
        chapters: book
            .iter()
            .filter_map(just_chapters)
            .map(|ch| FullChapter { src: ch })
            .collect(),
        print: PrintPage { book },
        assets: Vec::new(),
        sidebar: Sidebar {},
    }
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
        self.assets
            .par_iter()
            .try_for_each(|asset| asset.write_to(dest_dir))
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Asset {
    OnDisk {
        /// The asset's location on disk.
        src_filename: PathBuf,
        /// Where it should be written (relative to the output directory).
        destination: PathBuf,
    },
    FromTheme {
        /// The asset's path, relative to the `themes/static/` directory.
        filename: PathBuf,
        content: Vec<u8>,
    },
}

impl Asset {
    fn write_to(&self, destination_dir: &Path) -> Result<(), Error> {
        match self {
            Asset::OnDisk {
                src_filename,
                destination,
            } => {
                std::fs::copy(src_filename, destination_dir.join(destination))?;
                Ok(())
            },
            Asset::FromTheme { .. } => unimplemented!(),
        }
    }
}

/// Normal content.
struct FullChapter<'a> {
    src: &'a Chapter,
}

impl<'a> FullChapter<'a> {
    fn render(
        &self,
        hbs: &Handlebars,
        sidebar: &Sidebar,
        cfg: &Config,
        ctx: &RenderContext,
    ) -> Result<(), Error> {
        let rendered =
            render_chapter(self.src, hbs, sidebar, cfg, &ctx.config)?;

        unimplemented!()
    }
}

/// A page meant to show the entire document so it can be printed.
struct PrintPage<'a> {
    book: &'a Book,
}

impl<'a> PrintPage<'a> {
    fn render(
        &self,
        hbs: &Handlebars,
        sidebar: &Sidebar,
        cfg: &Config,
        ctx: &RenderContext,
    ) -> Result<(), Error> {
        let print_context = PrintContext {
            html_config: cfg,
            book_config: &ctx.config.book,
            sidebar,
        };
        let _rendered = hbs.render("layouts/chapter.hbs", &print_context)?;

        unimplemented!()
    }
}

fn render_chapter(
    chapter: &Chapter,
    hbs: &Handlebars,
    sidebar: &Sidebar,
    cfg: &Config,
    book_cfg: &mdbook::Config,
) -> Result<String, Error> {
    let body = md_to_html(&chapter.content);
    let context = Context {
        chapter: ChapterInfo { content: &body },
        html_config: cfg,
        book_config: &book_cfg.book,
        sidebar,
    };

    hbs.render("layouts/chapter.hbs", &context)
        .chain_err(|| format!("Unable to render \"{}\"", chapter.name))
}

fn md_to_html(src: &str) -> String {
    let mut buffer = String::with_capacity(src.len());
    pulldown_cmark::html::push_html(&mut buffer, Parser::new(src));

    buffer
}

fn register_default_helpers(_hbs: &mut Handlebars) {}

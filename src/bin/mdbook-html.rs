use mdbook::{
    errors::{Error, ResultExt},
    renderer::{RenderContext, Renderer as _},
    MDBook,
};
use mdbook_html::Renderer;
use std::{io, path::PathBuf};
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let args = Args::from_args();

    if let Err(e) = run(args) {
        log::error!("Error: {}", e);

        for cause in e.iter().skip(1) {
            log::error!("\tCaused By: {}", cause);
        }

        if let Some(bt) = e.backtrace() {
            log::error!("{:?}", bt);
        }
    }
}

fn run(args: Args) -> Result<(), Error> {
    let ctx: RenderContext = if args.standalone {
        let md = MDBook::load(dunce::canonicalize(&args.root)?)?;
        let destination = md.build_dir_for("linkcheck");
        RenderContext::new(md.root, md.book, md.config, destination)
    } else {
        serde_json::from_reader(io::stdin())
            .chain_err(|| "Unable to parse RenderContext")?
    };

    Renderer.render(&ctx)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, StructOpt)]
struct Args {
    #[structopt(parse(from_os_str), default_value = ".")]
    root: PathBuf,
    #[structopt(short = "s", long = "standalone")]
    standalone: bool,
}

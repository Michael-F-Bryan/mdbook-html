use crate::Config;
use handlebars::Handlebars;
use mdbook::{
    errors::{Error, ResultExt},
    renderer::RenderContext,
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
        register_default_helpers(&mut hbs);
        crate::themes::register(&mut hbs, &cfg)
            .chain_err(|| "Unable to register templates")?;
        log::debug!("Set up the handlebars renderer");

        unimplemented!()
    }
}

fn register_default_helpers(_hbs: &mut Handlebars) {}

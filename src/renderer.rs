use handlebars::Handlebars;
use mdbook::{errors::Error as MdError, renderer::RenderContext};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Renderer;

impl mdbook::Renderer for Renderer {
    fn name(&self) -> &str { "html-alt" }

    fn render(&self, _ctx: &RenderContext) -> Result<(), MdError> {
        let mut hbs = Handlebars::new();
        register_default_helpers(&mut hbs);

        unimplemented!()
    }
}

fn register_default_helpers(_hbs: &mut Handlebars) {}

use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderError,
};
use mdbook::renderer::RenderContext;
use serde_json::Value;
use std::path::{Path, PathBuf, StripPrefixError};

/// Registers all handlebars helpers.
pub fn register(hbs: &mut Handlebars, ctx: &RenderContext) {
    hbs.register_helper(
        "static",
        Box::new(Static {
            destination_dir: ctx.destination.clone(),
        }),
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct Static {
    destination_dir: PathBuf,
}

impl HelperDef for Static {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _hbs: &'reg Handlebars,
        ctx: &'rc Context,
        _rc: &mut handlebars::RenderContext<'reg>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let path = h
            .param(0)
            .map(|v| v.value())
            .ok_or(RenderError::new("No path provided"))?;
        let path = path
            .as_str()
            .ok_or(RenderError::new("The path must be a string"))?;

        let current_file = lookup(ctx, &["common", "current_file"])
            .and_then(|value| value.as_str())
            .ok_or(RenderError::new(
            "Unable to resolve `common.current_file` in the current context",
        ))?;

        let current_file = Path::new(current_file);

        let resolved =
            path_to_some_parent_dir(current_file, &self.destination_dir)
                .map_err(|e| RenderError::new(e.to_string()))?
                .join(path);
        let resolved = resolved.display().to_string();
        out.write(&resolved)?;

        Ok(())
    }
}

fn path_to_some_parent_dir(
    from: &Path,
    base: &Path,
) -> Result<PathBuf, StripPrefixError> {
    let relative_from = from.strip_prefix(base)?;
    let number_of_dot_dots = relative_from.components().skip(1).count();

    let mut resolved = PathBuf::new();
    for _ in 0..number_of_dot_dots {
        resolved.push("..");
    }

    Ok(resolved)
}

fn lookup<'ctx>(ctx: &'ctx Context, path: &[&str]) -> Option<&'ctx Value> {
    let mut data = ctx.data();

    for index in path {
        data = data.get(index)?;
    }

    Some(data)
}

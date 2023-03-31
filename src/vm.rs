use std::path::Path;
use std::sync::Arc;

use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, Source, Sources, Vm};

use crate::context::{Context, Shell};

pub fn vm_from_sources(path: &Path) -> Result<Vm, rune::Error> {
    // TODO: have a debug version of this with println included.
    let mut context = rune::Context::with_config(/*stdio=*/ false)?;

    let module = generate_module()?;
    context.install(&module)?;

    let runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    sources.insert(Source::from_path(path)?);

    // TODO: make diagnostics optional.
    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;
    Ok(Vm::new(runtime, Arc::new(unit)))
}

fn generate_module() -> Result<rune::Module, rune::compile::ContextError> {
    let mut module = rune::Module::default();

    module.ty::<Context>()?;
    module.async_inst_fn("hostname", Context::hostname)?;
    module.inst_fn("shell", Context::shell)?;
    module.async_inst_fn("username", Context::username)?;

    module.ty::<Shell>()?;
    module.inst_fn("to_string", Shell::to_string)?;

    Ok(module)
}

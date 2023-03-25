use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, FromValue, Hash, Source, Sources, Vm};
use std::path::Path;
use std::sync::Arc;

use context::Context;
mod context;
mod data;

fn generate_module() -> Result<rune::Module, rune::compile::ContextError> {
    let mut module = rune::Module::default();

    module.ty::<Context>()?;
    module.async_inst_fn("hostname", Context::hostname)?;

    //load_rune_async_function!(module, "get_hostname", data::hostname::hostname);

    Ok(module)
}

#[tokio::main]
async fn main() -> rune::Result<()> {
    let rune_entrypoint = Hash::type_hash(["generate_prompt"]);

    let mut context = rune::Context::with_default_modules()?;

    let module = generate_module().expect("setup error");
    context.install(&module)?;

    let runtime = Arc::new(context.runtime());

    let mut sources = Sources::new();
    sources.insert(Source::from_path(Path::new("src/prompt.rn"))?);

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
    let mut vm = Vm::new(runtime, Arc::new(unit));

    let output = vm
        .async_call(rune_entrypoint, (context::Context::new("bash".into(), 30),))
        .await?;
    let output = String::from_value(output)?;

    println!("{}", output);
    Ok(())
}

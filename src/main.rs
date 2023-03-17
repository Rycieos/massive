use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, FromValue, Hash, Source, Sources, Vm};
use std::path::Path;
use std::sync::Arc;

mod data;

macro_rules! load_rune_async_function {
    ($module:expr, $name:expr, $func:expr) => {{
        match $module.async_function(&[$name], $func) {
            Ok(()) => (),
            // TODO: panic if allowed.
            Err(_) => log::warn!("Failed to intitalize API function '{}'.", $name),
        }
    }};
}

fn generate_module() -> rune::Module {
    let mut module = rune::Module::default();

    load_rune_async_function!(module, "get_hostname", data::hostname::hostname);

    module
}

#[tokio::main]
async fn main() -> rune::Result<()> {
    let rune_entrypoint = Hash::type_hash(["generate_prompt"]);

    let mut context = Context::with_default_modules()?;

    let module = generate_module();
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

    let output = vm.async_call(rune_entrypoint, ()).await?;
    let output = String::from_value(output)?;

    println!("{}", output);
    Ok(())
}

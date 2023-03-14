use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, FromValue, Hash, Source, Sources, Vm};
use std::path::Path;
use std::sync::Arc;

async fn get_hostname() -> String {
    "jaguar".into()
}

#[tokio::main]
async fn main() -> rune::Result<()> {
    let rune_entrypoint = Hash::type_hash(["generate_prompt"]);

    let mut context = Context::with_default_modules()?;

    let mut module = rune::Module::default();
    module.async_function(&["get_hostname"], get_hostname)?;
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

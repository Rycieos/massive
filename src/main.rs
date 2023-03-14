fn get_hostname() -> rhai::ImmutableString {
    "jaguar".into()
}

pub fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    let mut engine = rhai::Engine::new();
    engine.register_fn("get_hostname", get_hostname);

    let ast = engine.compile_file("src/prompt.rhai".into())?;

    let mut scope = rhai::Scope::new();

    let result = engine.call_fn::<String>(&mut scope, &ast, "generate_prompt", ())?;

    println!("result: '{}'", result);

    Ok(())
}

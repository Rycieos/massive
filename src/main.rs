use clap::Parser;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, FromValue, Hash, Source, Sources, Vm};
use std::path::Path;
use std::sync::Arc;

use context::Context;
mod context;
mod data;

#[derive(Parser, Debug)]
#[clap(subcommand_required = true, arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Init {
        #[clap(subcommand)]
        shell: Shell,
    },
    Server,
    Prompt,
}

#[derive(clap::Subcommand, Debug)]
enum Shell {
    Bash,
}

#[tokio::main]
async fn main() -> rune::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Init { shell } => {
            println!("{:?}", shell);
        }
        Command::Server => {
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

            loop {
                let bytes = std::fs::read("/tmp/massive_in.fifo")?;

                if bytes.len() < 2 {
                    continue;
                }
                let mesg_type = bytes[0];
                let payload = String::from_utf8_lossy(&bytes[1..]);

                match mesg_type {
                    1 => { // hello
                    }
                    2 => {
                        // bye
                        return Ok(());
                    }
                    3 => {
                        // prompt request
                        // TODO break out the prompt central part into a method.
                        //println!("{}", payload);

                        let sections: Vec<&str> = payload.split('\x1f').collect();

                        if sections.len() != 10 {
                            log::error!("Bad prompt request: {}", payload);
                            continue;
                        }

                        let client_id = sections[0];
                        let resp_fifo = sections[1];
                        // TODO: make shell an Enum.
                        let shell = sections[2];
                        let terminal_width = sections[3];
                        let exit_status = sections[4];
                        let pipe_status = sections[5];
                        let jobs_running = sections[6];
                        let jobs_sleeping = sections[7];
                        let current_dir = sections[8];
                        let env_vars = sections[9];

                        // TODO: unsafe!
                        let context = context::Context::new(
                            shell.to_string(),
                            terminal_width.parse::<u16>()?,
                            exit_status.parse::<i32>()?,
                            pipe_status
                                .split(" ")
                                .map(str::parse::<i32>)
                                .map(Result::unwrap)
                                .collect(),
                            jobs_running.parse::<u16>()?,
                            jobs_sleeping.parse::<u16>()?,
                            current_dir.to_string(),
                            context::split_env_vars(shell, env_vars),
                        );
                        let output = vm.async_call(rune_entrypoint, (context,)).await?;

                        let output = String::from_value(output)?;
                        //println!("{}", output);
                        // TODO: this creates the file if not exists; we don't really want that.
                        std::fs::write(resp_fifo, output)?;
                    }
                    // TODO: timer start
                    //4 => {  // log time for client
                    _ => {
                        log::error!("passed message is invalid!");
                    }
                };
            }
        }
        Command::Prompt => {
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

            //let output = vm.async_call(rune_entrypoint, (context::Context::new("bash".into(), 30),)).await?;
            //let output = String::from_value(output)?;

            //println!("{}", output);
        }
    };
    Ok(())
}

fn generate_module() -> Result<rune::Module, rune::compile::ContextError> {
    let mut module = rune::Module::default();

    module.ty::<Context>()?;
    module.async_inst_fn("hostname", Context::hostname)?;
    module.async_inst_fn("username", Context::username)?;

    //load_rune_async_function!(module, "get_hostname", data::hostname::hostname);

    Ok(module)
}

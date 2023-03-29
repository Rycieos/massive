use std::collections::HashMap;
use std::path::Path;

use clap::Parser;
use rune::{FromValue, Hash};

use vm::vm_from_sources;

mod context;
mod data;
mod prompt_request;
mod server;
mod vm;

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
            server::server().await?;
        }
        Command::Prompt => {
            let rune_entrypoint = Hash::type_hash(["generate_prompt"]);
            let mut vm = vm_from_sources(Path::new("src/prompt.rn"))?;

            let env_vars: HashMap<String, String> = std::env::vars().collect();

            let context = crate::prompt_request::parse_prompt_request(
                "bash",
                "30",
                "0",
                "0 0",
                "0",
                "0",
                "",
                Some(env_vars),
                None,
            )?;
            let output = vm.async_call(rune_entrypoint, (context,)).await?;
            let output = String::from_value(output)?;

            println!("{}", output);
        }
    };
    Ok(())
}

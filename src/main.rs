use std::collections::HashMap;
use std::path::Path;

use clap::Parser;
use rune::{FromValue, Hash};

use crate::context::{Context, Shell};
use crate::vm::vm_from_sources;

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
        #[arg(value_enum)]
        shell: Shell,
    },
    Server,
    Prompt {
        // TODO: the env var is not parsed correctly.
        #[arg(long, value_enum, env="SHELL")]
        shell: Shell,
        #[arg(long)]
        terminal_width: Option<u16>,
        #[arg(long)]
        exit_status: Option<i32>,
        #[arg(long, value_delimiter=' ')]
        pipe_status: Vec<i32>,
        #[arg(long)]
        jobs_running: Option<u16>,
        #[arg(long)]
        jobs_sleeping: Option<u16>,
        #[arg(long)]
        current_dir: Option<String>,
    },
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
        Command::Prompt {
            shell,
            terminal_width,
            exit_status,
            pipe_status,
            jobs_running,
            jobs_sleeping,
            current_dir,
        } => {
            let rune_entrypoint = Hash::type_hash(["generate_prompt"]);
            let mut vm = vm_from_sources(Path::new("src/prompt.rn"))?;

            // TODO: this will panic on any non-Unicode values.
            let env_vars: HashMap<String, String> = std::env::vars().collect();

            // TODO: better defaults.
            // Do we want to store Options so we can tell between 0 and None?
            let context = Context::new(
                shell,
                terminal_width.unwrap_or(0),
                exit_status.unwrap_or(0),
                pipe_status,
                jobs_running.unwrap_or(0),
                jobs_sleeping.unwrap_or(0),
                current_dir.unwrap_or_else(|| {
                    std::env::current_dir().unwrap().into_os_string().into_string().unwrap()
                }),
                env_vars,
            );
            let output = vm.async_call(rune_entrypoint, (context,)).await?;
            let output = String::from_value(output)?;

            println!("{}", output);
        }
    };
    Ok(())
}

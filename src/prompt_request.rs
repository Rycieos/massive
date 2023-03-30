use std::collections::HashMap;

use crate::context::{Context, Shell};

pub fn parse_prompt_request(
    shell: &str,
    terminal_width: &str,
    exit_status: &str,
    pipe_status: &str,
    jobs_running: &str,
    jobs_sleeping: &str,
    current_dir: &str,
    env_vars_str: &str,
) -> rune::Result<Context> {
    let shell: Shell = Shell::from_str(shell);

    let mut pipe_statuses = Vec::new();
    for status in pipe_status.split(" ") {
        pipe_statuses.push(status.parse::<i32>()?);
    }

    let env_vars = split_env_vars(&shell, env_vars_str);

    let context = Context::new(
        shell,
        terminal_width.parse::<u16>()?,
        exit_status.parse::<i32>()?,
        pipe_statuses,
        jobs_running.parse::<u16>()?,
        jobs_sleeping.parse::<u16>()?,
        current_dir.to_string(),
        env_vars,
    );

    Ok(context)
}

fn split_env_vars(shell: &Shell, payload: &str) -> HashMap<String, String> {
    match shell {
        Shell::Bash => split_bash_vars(payload),
        Shell::Unknown => HashMap::new(),
    }
}

fn split_bash_vars(payload: &str) -> HashMap<String, String> {
    let mut env_vars = HashMap::new();

    for var in payload.split("declare -x ") {
        if var.len() > 0 {
            match var.split_once("=\"") {
                Some((key, value)) => {
                    let value = match value.strip_suffix('\n') {
                        Some(stripped) => stripped,
                        None => value,
                    };
                    let value = match value.strip_suffix('"') {
                        Some(stripped) => stripped,
                        None => value,
                    };
                    let value = value.replace("\\\"", "\"");
                    env_vars.insert(key.to_string(), value);
                }
                None => continue,
            }
        }
    }

    env_vars
}

use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, rune::Any)]
pub struct Context {
    global: HashMap<&'static str, String>,
    client: HashMap<&'static str, String>,
    instance: HashMap<&'static str, String>,
    shell: String,
    width: u16,
    exit_status: i32,
    pipe_status: Vec<i32>,
    jobs_running: u16,
    jobs_sleeping: u16,
    current_dir: String,
    pub envs: HashMap<String, String>,
}

macro_rules! with_cache {
    ($cache:expr, $key:expr, $func:expr) => {
        if $cache.contains_key($key) {
            return $cache.get($key).unwrap();
        }

        let value = $func;
        $cache.insert($key, value);
        return $cache.get($key).unwrap();
    };
}

impl Context {
    pub fn new(
        shell: String,
        width: u16,
        exit_status: i32,
        pipe_status: Vec<i32>,
        jobs_running: u16,
        jobs_sleeping: u16,
        current_dir: String,
        envs: HashMap<String, String>,
    ) -> Self {
        Context {
            // TODO: actual caching.
            global: HashMap::new(),
            client: HashMap::new(),
            instance: HashMap::new(),
            shell,
            width,
            exit_status,
            pipe_status,
            jobs_running,
            jobs_sleeping,
            current_dir,
            envs,
        }
    }

    pub async fn hostname(&mut self) -> &str {
        with_cache!(
            self.global,
            "hostname",
            crate::data::hostname::hostname(self).await
        );
    }

    pub async fn username(&mut self) -> &str {
        with_cache!(
            self.global,
            "username",
            crate::data::username::username(self).await
        );
    }
}

pub fn split_env_vars(shell: &str, payload: &str) -> HashMap<String, String> {
    if shell == "bash" {
        return split_bash_vars(payload);
    }

    HashMap::new()
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

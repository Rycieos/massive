use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq, rune::Any)]
pub enum Shell {
    Bash,
    Unknown,
}

impl Shell {
    pub fn from_str(input: &str) -> Self {
        let input = match input.rsplit_once('/') {
            Some((_, end)) => end,
            None => input,
        };

        match input.to_lowercase().as_ref() {
            "bash" => Self::Bash,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, rune::Any)]
pub struct Context {
    global: HashMap<&'static str, String>,
    client: HashMap<&'static str, String>,
    instance: HashMap<&'static str, String>,
    shell: Shell,
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
        shell: Shell,
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

    pub fn shell(&self) -> Shell {
        self.shell
    }

    pub async fn username(&mut self) -> &str {
        with_cache!(
            self.global,
            "username",
            crate::data::username::username(self).await
        );
    }
}

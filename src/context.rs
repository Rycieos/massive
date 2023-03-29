use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub enum Shell {
    Bash,
    Unknown,
}

impl Shell {
    pub fn from_str(from: &str) -> Self {
        match from {
            "bash" => Self::Bash,
            _ => Self::Unknown,
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

    pub async fn username(&mut self) -> &str {
        with_cache!(
            self.global,
            "username",
            crate::data::username::username(self).await
        );
    }
}

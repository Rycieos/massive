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
    envs: HashMap<String, String>,
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
    pub fn new(shell: String, width: u16) -> Self {
        Context {
            global: HashMap::new(),
            client: HashMap::new(),
            instance: HashMap::new(),
            shell,
            width,
            exit_status: 0,
            pipe_status: vec![0],
            jobs_running: 0,
            jobs_sleeping: 0,
            current_dir: "".to_string(),
            envs: HashMap::new(),
        }
    }

    pub async fn hostname(&mut self) -> &str {
        with_cache!(
            self.instance,
            "hostname",
            crate::data::hostname::hostname(self).await
        );
    }
}

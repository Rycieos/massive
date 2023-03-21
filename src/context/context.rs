use crate::context::cache::GlobalCache;

pub struct Context {
    global: GlobalCache,
}

impl Context {
    fn new() -> Self {
        Context {
            global: GlobalCache::new(),
        }
    }

    pub async fn hostname(&self) -> Option<String> {
        if self.global.hostname.is_some() && self.global.hostname {
            None
    }
}

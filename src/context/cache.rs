
struct TimedCache<T> {
    object: T,
    expiration: std::time::Instant,
}

struct GlobalCache {
    hostname: Option<TimedCache<String>>,
}

impl GlobalCache {
    fn new() -> Self {
        GlobalCache {
            hostname: None,
        }
    }
}

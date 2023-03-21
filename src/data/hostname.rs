use std::ffi::OsString;

pub async fn hostname() -> Option<String> {
    let os_hostname: OsString = gethostname::gethostname();

    let host = match os_hostname.into_string() {
        Ok(host) => host,
        Err(bad) => {
            log::warn!("hostname is not valid UTF-8!\n{:?}", bad);
            return None;
        }
    };

    Some(host)
}

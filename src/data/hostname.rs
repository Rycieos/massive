use std::ffi::OsString;

use crate::context::Context;

pub async fn hostname(_context: &Context) -> String {
    let os_hostname: OsString = gethostname::gethostname();

    match os_hostname.into_string() {
        Ok(host) => host,
        Err(bad) => {
            log::warn!("hostname is not valid UTF-8!\n{:?}", bad);
            "".to_string()
        }
    }
}

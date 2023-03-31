use std::ffi::OsString;

use nix::unistd::gethostname;

use crate::context::Context;

pub async fn hostname(_context: &Context) -> String {
    let os_hostname: OsString = gethostname().unwrap_or("".into());

    match os_hostname.into_string() {
        Ok(host) => host,
        Err(bad) => {
            log::warn!("hostname is not valid UTF-8!\n{:?}", bad);
            "".to_string()
        }
    }
}

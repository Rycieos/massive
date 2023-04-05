use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use directories::BaseDirs;
use nix::sys::stat::Mode;
use nix::unistd;

pub fn get_home_dir() -> Option<PathBuf> {
    BaseDirs::new().map(|bd| bd.home_dir().to_owned())
}

pub fn get_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("MOST_CONFIG") {
        log::debug!("MOST_CONFIG is set: {}", &path);
        Some(PathBuf::from(path))
    } else {
        BaseDirs::new()
            .map(|bd| bd.config_dir().to_owned())
            .map(|mut home| {
                home.push(Path::new("most/config.toml"));
                home
            })
    }
}

pub fn get_state_dir() -> Option<PathBuf> {
    if let Ok(path) = env::var("MOST_STATE_DIR") {
        log::debug!("MOST_STATE_DIR is set: {}", &path);
        Some(PathBuf::from(path))
    } else {
        let state_dir: Option<PathBuf> =
            BaseDirs::new().and_then(|bd| bd.state_dir().map(|dir| dir.to_owned()));

        match state_dir {
            Some(mut dir) => {
                dir.push("most");
                Some(dir)
            }
            None => {
                // Need a place to put the state, so use the config location.
                log::debug!("No application state directory found");
                get_config_path().map(|mut config| {
                    config.pop();
                    config.push("state");
                    config
                })
            }
        }
    }
}

fn get_in_fifo() -> Option<PathBuf> {
    get_state_dir().map(|mut dir| {
        dir.push("server_in.fifo");
        dir
    })
}

pub fn get_or_create_in_fifo() -> PathBuf {
    let in_fifo = get_in_fifo().expect("Can't find input FIFO location");
    log::debug!("in_fifo: {:?}", in_fifo);

    if !in_fifo.exists() {
        let parent = in_fifo.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create state directory");
        }
        unistd::mkfifo(&in_fifo, Mode::S_IRWXU).expect("Failed to create FIFO");
    }

    in_fifo
}

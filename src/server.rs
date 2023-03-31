use std::fs;
use std::path::Path;

use pidlock::Pidlock;
use rune::FromValue;

use crate::filesystem::get_or_create_in_fifo;
use crate::prompt_request::parse_prompt_request;
use crate::vm::vm_from_sources;

pub async fn server() -> rune::Result<()> {
    let mut lock = Pidlock::new("/home/mark/.config/massive/server/pid.lock");
    lock.acquire().expect("Failed to acquire lock, exiting!");

    let rune_entrypoint = rune::Hash::type_hash(["generate_prompt"]);
    let mut vm = vm_from_sources(Path::new("src/prompt.rn"))?;

    let in_fifo = get_or_create_in_fifo();

    loop {
        let Ok(bytes) = fs::read(&in_fifo) else {
            log::error!("failed to read from input FIFO");
            continue;
        };

        if bytes.len() < 2 {
            log::error!("message too short");
            continue;
        }
        let mesg_type = bytes[0];
        let payload = String::from_utf8_lossy(&bytes[1..]);

        match mesg_type {
            1 => { // hello
            }
            2 => {
                // bye
                break;
            }
            3 => {
                // prompt request
                let sections: Vec<&str> = payload.split('\x1f').collect();

                if sections.len() != 10 {
                    log::error!("prompt request did not have 10 sections");
                    continue;
                }

                let _client_id = sections[0].to_string();
                let resp_fifo = sections[1].to_string();

                let Ok(context) = parse_prompt_request(
                    sections[2],
                    sections[3],
                    sections[4],
                    sections[5],
                    sections[6],
                    sections[7],
                    sections[8],
                    sections[9],
                ) else {
                    log::error!("failed to parse message into context");
                    continue;
                };

                // TODO: handle these results.
                let output = vm.async_call(rune_entrypoint, (context,)).await?;
                let output = String::from_value(output)?;

                // TODO: this creates the file if not exists; we don't really want that.
                // TODO: handle these results.
                std::fs::write(resp_fifo, output)?;
            }
            // TODO: timer start
            //4 => {  // log time for client
            _ => {
                log::error!("message type is invalid");
            }
        };
    }

    lock.release().expect("Failed to release lock");
    Ok(())
}

use crate::context::Context;

pub async fn username(context: &Context) -> String {
    let username = match context.envs.get("LOGNAME") {
        Some(value) => value,
        None => {
            match context.envs.get("USER") {
                Some(value) => value,
                None => {
                    match context.envs.get("USERNAME") {
                        Some(value) => value,
                        None => "",
                    }
                }
            }
        }
    };

    username.to_string()
}

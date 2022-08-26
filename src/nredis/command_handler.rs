use super::command_types::{self, RESPValue};

pub fn handle_command(command: &command_types::RESPValue) -> command_types::RESPValue {
    match command {
        command_types::RESPValue::String(x) => handle_single_command(x),
        command_types::RESPValue::Error(_) => todo!(),
        command_types::RESPValue::Array(items) => {
            RESPValue::Array(items.iter().map(handle_command).collect())
        }
    }
}

fn handle_single_command(message: &str) -> command_types::RESPValue {
    let mut message_parts = message.split(' ');
    let command = message_parts.next().unwrap();
    let arguments: Vec<&str> = message_parts.collect();

    match command {
        "ECHO" => handle_echo(&arguments),
        _ => command_types::RESPValue::String(String::from("Unknown command!")),
    }
}

fn handle_echo(arguments: &[&str]) -> command_types::RESPValue {
    RESPValue::String(arguments.join(" "))
}

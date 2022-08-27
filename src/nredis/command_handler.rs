use super::command_types::{self, RESPValue, RESPError};

pub fn handle_command(command: &command_types::RESPValue) -> command_types::RESPValue {
    match command {
        command_types::RESPValue::String(x) => handle_string(x),
        command_types::RESPValue::Error(_) => todo!(),
        command_types::RESPValue::Array(items) => {
            let mut iterator = items.iter();
            let command = match iterator.next().unwrap() {
                command_types::RESPValue::String(x) => x,
                _ => {return command_types::RESPValue::Error(RESPError{message: "Command needs to be defined as string".to_string()})}
            };
            let arguments: Vec<&RESPValue> = iterator.collect();
            handle_single_command(command, &arguments)
        }
    }
}

fn handle_string(message: &str) -> command_types::RESPValue {
    let mut message_parts = message.split(' ');
    let command = message_parts.next().unwrap();
    let arguments: Vec<&str> = message_parts.collect();

    handle_single_command(command, &[])
}

fn handle_single_command(command: &str, arguments: &[&RESPValue]) -> command_types::RESPValue {
    match command.to_lowercase().as_str() {
        "echo" => handle_echo(&arguments),
        "ping" => handle_ping(&arguments),
        _ => command_types::RESPValue::String(String::from("Unknown command!")),
    }
}

fn handle_echo(arguments: &[&RESPValue]) -> command_types::RESPValue {
    let string_arguments: Vec<&str> = arguments.iter().map(|&x| match x { 
        command_types::RESPValue::String(y) => y, 
        _ => ""
    }).collect();

    RESPValue::String(string_arguments.join(" "))
}

fn handle_ping(_: &[&RESPValue]) -> command_types::RESPValue {
    RESPValue::String("PONG".to_string())
}

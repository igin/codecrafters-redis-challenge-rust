use super::command_types::{RESPValue, RESPError, State};

pub fn handle_command(command: &RESPValue, state: &mut State) -> RESPValue {
    match command {
        RESPValue::String(x) => handle_string(x, state),
        RESPValue::Error(_) => todo!(),
        RESPValue::Array(items) => {
            let mut iterator = items.iter();
            let command = match iterator.next().unwrap() {
                RESPValue::String(x) => x,
                _ => {return RESPValue::Error(RESPError{message: "Command needs to be defined as string".to_string()})}
            };
            let arguments: Vec<&RESPValue> = iterator.collect();
            handle_single_command(command, &arguments, state)
        }
    }
}

fn handle_string(message: &str, state: &mut State) -> RESPValue {
    handle_single_command(message, &[], state)
}

fn handle_single_command(command: &str, arguments: &[&RESPValue], state: &mut State) -> RESPValue {
    match command.to_lowercase().as_str() {
        "echo" => handle_echo(&arguments),
        "ping" => handle_ping(&arguments),
        "set" => handle_set(&arguments, state),
        "get" => handle_get(&arguments, &state),
        _ => RESPValue::String(String::from("Unknown command!")),
    }
}

fn handle_echo(arguments: &[&RESPValue]) -> RESPValue {
    let string_arguments: Vec<&str> = arguments.iter().map(|&x| match x { 
        RESPValue::String(y) => y, 
        _ => ""
    }).collect();

    RESPValue::String(string_arguments.join(" "))
}

fn handle_ping(_: &[&RESPValue]) -> RESPValue {
    RESPValue::String("PONG".to_string())
}

fn handle_set(arguments: &[&RESPValue], state: &mut State) -> RESPValue {
    let string_arguments: Vec<&str> = arguments.iter().map(|&x| match x { 
        RESPValue::String(y) => y, 
        _ => ""
    }).collect();

    let mut arguments_iter = string_arguments.iter();
    let key = arguments_iter.next().unwrap().to_string();
    let value = arguments_iter.next().unwrap().to_string();

    state.map.insert(key, value);
    RESPValue::String("OK".to_string())
}

fn handle_get(arguments: &[&RESPValue], state: &State) -> RESPValue {
    let string_arguments: Vec<&str> = arguments.iter().map(|&x| match x { 
        RESPValue::String(y) => y, 
        _ => ""
    }).collect();

    let key_to_find = string_arguments.iter().next().unwrap();
    let value = state.map.get(&key_to_find.to_string()).expect("Value not found");
    RESPValue::String(value.to_string())
}

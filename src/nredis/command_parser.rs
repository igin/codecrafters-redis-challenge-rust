use std::io::{BufRead, Read};

use super::command_types::{self, RESPError, RESPValue};

pub fn parse_next_command(reader: &mut impl BufRead) -> Option<command_types::RESPValue> {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Couldn't read a new line");

    match line.chars().next() {
        Some('+') => Some(RESPValue::String(parse_simple_string(&line))),
        Some('-') => Some(RESPValue::Error(parse_error(&line))),
        Some('*') => Some(RESPValue::Array(parse_array(&line, reader))),
        Some('$') => Some(RESPValue::String(parse_bulk_string(&line, reader))),
        Some(_) => None,
        None => None,
    }
}

fn parse_simple_string(line: &str) -> String {
    line.strip_prefix('+').unwrap().trim().to_string()
}

fn parse_error(line: &str) -> RESPError {
    RESPError {
        message: line.strip_prefix('-').unwrap().trim().to_string(),
    }
}

fn parse_array(line: &str, reader: &mut impl BufRead) -> Vec<RESPValue> {
    let mut lines_to_parse: u16 = line.strip_prefix('*').unwrap().trim().parse().unwrap();
    let mut values: Vec<RESPValue> = Vec::new();
    while lines_to_parse > 0 {
        let value = parse_next_command(reader).expect("Couldn't parse array element.");
        values.push(value);
        lines_to_parse -= 1;
    }
    values
}

fn parse_bulk_string(line: &str, reader: &mut impl BufRead) -> String {
    let bytes_to_parse: u64 = line.strip_prefix('$').unwrap().trim().parse().unwrap();

    let mut result = String::new();
    reader
        .take(bytes_to_parse)
        .read_to_string(&mut result)
        .expect("Couldn't read bytes to string");

    // consume the newline after the string
    let mut new_line = String::new();
    reader
        .read_line(&mut new_line)
        .expect("Couldn't read a new line at end of bulk string");

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn parsing_simple_string_works() {
        let input = "+SIMPLE_STUFF\r\n";
        let result = parse_next_command(&mut input.as_bytes());
        assert!(matches!(result, Some(RESPValue::String(..))));
    }

    #[test]
    fn parsing_multiple_lines() {
        let input = "+SIMPLE_STUFF\r\n+OTHER_STUFF";
        let stream = &mut input.as_bytes();
        let result = parse_next_command(stream);
        assert!(matches!(result, Some(RESPValue::String(..))));
        let result = parse_next_command(stream);
        assert!(matches!(result, Some(RESPValue::String(..))));
    }

    #[test]
    fn returns_none_if_no_command_is_found() {
        let stream = &mut Cursor::new("+SIMPLE_STUFF\r\n+OTHER_STUFF\r\n");
        let mut reader = BufReader::new(stream);

        let command = parse_next_command(&mut reader);
        println!("{command:?}");
        assert!(matches!(command, Some(RESPValue::String(..))));
        let command = parse_next_command(&mut reader);
        println!("{command:?}");
        assert!(matches!(command, Some(RESPValue::String(..))));
        let command = parse_next_command(&mut reader);
        println!("{command:?}");
        assert!(matches!(command, None));
    }

    #[test]
    fn parses_error() {
        let stream = &mut Cursor::new("-ERROR_MESSAGE\r\n");
        let mut reader = BufReader::new(stream);

        let command = parse_next_command(&mut reader);
        println!("{command:?}");
        assert!(matches!(command, Some(RESPValue::Error(RESPError { .. }))));
    }

    #[test]
    fn parses_array() {
        let lines = [
            "*7",
            "+MESSAGE 1",
            "+MESSAGE 2",
            "+MESSAGE 3",
            "+MESSAGE 4",
            "+MESSAGE 5",
            "$5",
            "hello",
            "$5",
            "hello",
            "",
        ];

        let stream = &mut Cursor::new(lines.join("\r\n"));
        let mut reader = BufReader::new(stream);
        let command = parse_next_command(&mut reader);
        println!("{command:?}");
    }
}

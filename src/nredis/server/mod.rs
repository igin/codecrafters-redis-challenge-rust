use std::io::{BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;

use super::command_parser::parse_next_command;
use super::{command_handler, resp_serializer};

pub struct Config<'a> {
    pub address: &'a str,
}

pub fn listen(config: &Config) {
    let listener = TcpListener::bind(config.address).unwrap();
    println!("Starting NRedis server on {:?}", config.address);
    for stream in listener.incoming() {
        thread::spawn(|| {
            match stream {
                Ok(socket) => {
                    let mut input_stream = socket.try_clone().unwrap();
                    let mut output_stream = socket.try_clone().unwrap();

                    handle_connection(&mut input_stream, &mut output_stream);
                }
                Err(e) => {
                    println!("couldn't accept client: {:?}", e);
                }
            };
        });
    }
}

fn handle_connection(input_stream: &mut impl Read, output_stream: &mut impl Write) {
    let mut reader = BufReader::new(input_stream);
    while let Some(command) = parse_next_command(&mut reader) {
        println!("Got command {command:?}");

        let response = command_handler::handle_command(&command);
        let serialized = resp_serializer::serialize_resp(&response) + "\r\n";
        println!("Responding with {serialized:?}");
        let written_size = output_stream
            .write(serialized.as_bytes())
            .expect("Could not write to output stream");

        if written_size == 0 {
            println!("Failed to write to stream");
            return;
        }

        output_stream.flush().expect("Couldn't flush.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod tcp_test;

    #[test]
    fn server_responds_to_multiple_ping_commands() {
        let mut mock_input_stream = tcp_test::MockTcpStream {
            read_data: b"PING\nPING\n".to_vec(),
            write_data: Vec::new(),
        };
        let mut mock_output_stream = tcp_test::MockTcpStream {
            read_data: b"PING\nPING\n".to_vec(),
            write_data: Vec::new(),
        };
        handle_connection(&mut mock_input_stream, &mut mock_output_stream);
        assert_eq!(mock_output_stream.write_data, b"+PONG\n+PONG\n")
    }
}

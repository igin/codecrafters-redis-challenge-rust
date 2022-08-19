use std::cmp::min;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    match listener.accept() {
        Ok((socket, _addr)) => {
            let mut input_stream = socket.try_clone().unwrap();
            let mut output_stream = socket.try_clone().unwrap();

            handle_connection(&mut input_stream, &mut output_stream);
        }
        Err(e) => {
            println!("couldn't accept client: {:?}", e);
        }
    };
}

fn handle_connection(input_stream: &mut impl Read, output_stream: &mut impl Write) {
    let reader = BufReader::new(input_stream);
    for line in reader.lines() {
        println!("Got command: {line:?}");
        let response = handle_request(&line.unwrap());

        print!("Writing response: {response:?}");

        let amount = output_stream
            .write(response.as_bytes())
            .expect("Could not write to output stream");

        if amount == 0 {
            panic!("Nothing was written to the stream")
        }
    }
}

fn handle_request(command: &str) -> &'static str {
    match command {
        "PING" => "+PONG\n",
        &_ => "+PONG\n",
    }
}

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        self.read_data.drain(..size);
        Ok(size)
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_data.append(&mut Vec::from(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{handle_connection, handle_request, MockTcpStream};

    #[test]
    fn server_returns_pong_on_ping() {
        let response = handle_request("PING");
        assert_eq!(response, "+PONG\n");
    }

    #[test]
    fn server_returns_error_on_unknown_command() {
        let response = handle_request("UNKNWON");
        assert_eq!(response, "ERROR\n");
    }

    #[test]
    fn server_responds_to_multiple_ping_commands() {
        let mut mock_input_stream = MockTcpStream {
            read_data: b"PING\nPING\n".to_vec(),
            write_data: Vec::new(),
        };
        let mut mock_output_stream = MockTcpStream {
            read_data: b"PING\nPING\n".to_vec(),
            write_data: Vec::new(),
        };
        handle_connection(&mut mock_input_stream, &mut mock_output_stream);
        assert_eq!(mock_output_stream.write_data, b"+PONG\n+PONG\n")
    }
}

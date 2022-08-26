use std::{
    cmp::min,
    io::{Read, Write},
};

pub struct MockTcpStream {
    pub read_data: Vec<u8>,
    pub write_data: Vec<u8>,
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

// todo: for now, we only deal with IPv4

use crate::parser::message::Message;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;

pub struct TcpController {
    buffer_size: usize,
}

impl TcpController {
    pub fn new(buffer_size: usize) -> Self {
        Self { buffer_size }
    }

    pub fn query(&self, encoded_msg: &mut Vec<u8>, server: &SocketAddr) -> Option<Vec<u8>> {
        Message::add_2_bytes_length_for_tcp(encoded_msg);

        let mut stream = TcpStream::connect(server).expect("couldn't connect to address");

        stream
            .write(&encoded_msg)
            .expect("could not send to server");

        // wait FOREVER
        self.wait_for_response(&mut stream)
    }

    fn wait_for_response(&self, stream: &mut TcpStream) -> Option<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; self.buffer_size];

        match stream.read(&mut buffer) {
            Ok(received) => {
                // leave 2 bytes length out
                let filled_buffer = &buffer[2..received];
                Some(filled_buffer.to_vec())
            }
            Err(_) => None,
        }
    }
}

// todo: for now, we only deal with IPv4

use std::net::SocketAddr;
use std::net::UdpSocket;

pub struct UdpController {
    buffer_size: usize,
    interface: SocketAddr,
}

impl UdpController {
    pub fn new(buffer_size: usize, interface: SocketAddr) -> Self {
        Self {
            buffer_size,
            interface,
        }
    }

    pub fn query(&self, encoded_msg: &Vec<u8>, server: &SocketAddr) -> Option<Vec<u8>> {
        let socket = self.create_socket();

        socket.connect(&server).expect("connect function failed");
        socket.send(&encoded_msg).expect("could not send to server");

        self.wait_for_response(&socket)
    }

    fn wait_for_response(&self, socket: &UdpSocket) -> Option<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; self.buffer_size];

        match socket.recv(&mut buffer) {
            Ok(received) => {
                let filled_buffer = &buffer[..received];
                Some(filled_buffer.to_vec())
            }
            Err(_) => None,
        }
    }

    fn create_socket(&self) -> UdpSocket {
        UdpSocket::bind(self.interface).expect("couldn't bind to address")
    }
}

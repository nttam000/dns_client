// todo: for now, we only deal with IPv4

use std::net::UdpSocket;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

pub struct UdpController {
    buffer_size: u16,
    interface: SocketAddr,
    server: SocketAddr
}

impl UdpController {
    pub fn new(buffer_size: u16, interface: SocketAddr, server: SocketAddr) -> Self {
        Self {
            buffer_size,
            interface,
            server
        }
    }

    pub fn send_query(&self, encoded_msg: &Vec<u8>) -> Option<Vec<u8>> {
        let socket = self.create_socket();

        socket.connect(&self.server).expect("connect function failed");

        socket.send(&encoded_msg).expect("could not send to server");
        socket.send(&encoded_msg).expect("could not send to server");

        // todo: use buffer_size instead of hardcode
        let mut buffer = [0; 1023];

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

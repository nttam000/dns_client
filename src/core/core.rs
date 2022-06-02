use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use crate::{DnsResult, DnsError};
use crate::net::udp_controller::UdpController;
use crate::parser::message::Message;

pub struct Core {
    udp_controller: UdpController,
}

impl Core {
    const DEFAULT_UDP_BUFFER: usize = 1024;

    // todo: what address should be used here
    const DEFAULT_INTERFACE_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

    pub fn new() -> Self {
        let interface = SocketAddr::new(
            Self::DEFAULT_INTERFACE_IP,
            0
        );

        Self {
            udp_controller: UdpController::new(
                Self::DEFAULT_UDP_BUFFER, interface)
        }
    }

    pub fn send_query(&self, domain: &str) -> Result<DnsResult, DnsError> {
        self.send_query_to_network_layer(&domain, &Self::get_default_server())
    }

    pub fn send_query_with_server(&self, domain: &str, server: &str)
    -> Result<DnsResult, DnsError> {
        let server_ip = Self::get_server_ip_from_string(&server);
        self.send_query_to_network_layer(&domain, &server_ip)
    }

    fn get_server_ip_from_string(server: &str) -> SocketAddr {
        match server.parse() {
            Ok(socket) => socket,
            Err(_) => panic!()
        }
    }

    fn get_default_server() -> SocketAddr {
        Self::get_server_ip_from_string("8.8.8.8:53")
    }

    fn send_query_to_network_layer(&self, domain: &str, server: &SocketAddr)
    -> Result<DnsResult, DnsError> {
        let msg = Message::new(domain);

        let encoded_msg = msg.encode();

        let encoded_response =
            self.udp_controller.send_query_to_server(&encoded_msg, &server);

        match encoded_response {
            Some(response) => {
                let msg = Message::parse(&response);
                let answers = msg.get_answers().get_resource_records();

                let mut result = Vec::new();
                for answer in answers {
                    result.push(answer.get_data().clone());
                }
                Ok(DnsResult::new(result)) //give up ownership of result
            }
            None => Err(DnsError::new(0))
        }
    }
}
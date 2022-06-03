use std::net::SocketAddr;
use crate::{DnsResult, DnsError};
use crate::net::udp_controller::UdpController;
use crate::parser::message::Message;
use crate::config::config_handler::CONFIG;

pub struct Core {
    udp_controller: UdpController,
    default_servers: Vec<SocketAddr>
}

impl Core {
    pub fn new() -> Self {
        let local_interface =
            Self::get_server_ip_from_string(&CONFIG.local_interface);

        let mut default_servers: Vec<SocketAddr> = Vec::new();
        for server in &CONFIG.default_servers {
            default_servers.push(Self::get_server_ip_from_string(server));
        }

        let udp_controller =
            UdpController::new(CONFIG.udp_buffer_size, local_interface);

        Self {
            udp_controller,
            default_servers
        }
    }

    pub fn send_query(&self, domain: &str) -> Result<DnsResult, DnsError> {
        let default_server = &self.get_default_server();
        match default_server {
            Some(server) => self.send_query_to_network_layer(&domain, server),
            None => panic!()
        }
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

    // todo: don't hard code like that (0)
    fn get_default_server(&self) -> Option<SocketAddr> {
        if self.default_servers.len() == 0 {
            return None;
        }
        Some(self.default_servers[0])
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
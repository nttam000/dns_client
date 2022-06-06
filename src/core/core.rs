use crate::config::config_handler::CONFIG;
use crate::net::tcp_controller::TcpController;
use crate::net::udp_controller::UdpController;
use crate::parser::message::Message;
use crate::{Answer, Error, Protocol};
use std::net::SocketAddr;

pub struct Core {
    udp_controller: UdpController,
    tcp_controller: TcpController,
    default_servers: Vec<SocketAddr>,
}

impl Core {
    pub fn new() -> Self {
        let local_interface = Self::get_server_ip_from_string(&CONFIG.local_interface);

        let mut default_servers: Vec<SocketAddr> = Vec::new();
        for server in &CONFIG.default_servers {
            default_servers.push(Self::get_server_ip_from_string(server));
        }

        let udp_controller = UdpController::new(CONFIG.udp_buffer_size, local_interface);
        let tcp_controller = TcpController::new(CONFIG.tcp_buffer_size);

        Self {
            udp_controller,
            tcp_controller,
            default_servers,
        }
    }

    pub fn query(&self, domain: &str) -> Result<Answer, Error> {
        let default_server = &self.get_default_server();
        let edns_enable = true;
        let protocol = Protocol::Udp;
        let tcp_fallback = true;

        match default_server {
            Some(server) => self.send_query(&domain, server, edns_enable, protocol, tcp_fallback),
            None => panic!(),
        }
    }

    pub fn query_with_options(
        &self,
        domain: &str,
        server_ip: &str,
        edns_enable: bool,
        protocol: Protocol,
        tcp_fallback: bool,
    ) -> Result<Answer, Error> {
        let server_ip = Self::get_server_ip_from_string(&server_ip);
        self.send_query(&domain, &server_ip, edns_enable, protocol, tcp_fallback)
    }

    fn send_query(
        &self,
        domain: &str,
        server: &SocketAddr,
        edns_enable: bool,
        protocol: Protocol,
        tcp_fallback: bool,
    ) -> Result<Answer, Error> {
        let mut msg = Message::new(domain);

        if edns_enable {
            msg.convert_to_edns_query();
        }

        let mut encoded_msg = msg.encode();

        // UDP
        if let Protocol::Udp = protocol {
            let encoded_response = self.udp_controller.query(&encoded_msg, &server);

            let result = Self::analyze_result(&encoded_response);

            if let Ok(_) = result {
                return result;
            } else {
                if !tcp_fallback {
                    return Err(Error::new(0));
                }
            }
        }

        // TCP and TCP fallback
        if protocol == Protocol::Tcp || (tcp_fallback && Message::is_truncated(&encoded_msg)) {
            let encoded_response = self.tcp_controller.query(&mut encoded_msg, &server);
            Self::analyze_result(&encoded_response)
        } else {
            Err(Error::new(0))
        }
    }

    fn analyze_result(encoded_response: &Option<Vec<u8>>) -> Result<Answer, Error> {
        if let Some(response) = encoded_response {
            let msg = Message::parse(&response);
            let answers = msg.get_answers().get_records();

            let mut result = Vec::new();
            for answer in answers {
                result.push(answer.get_data().clone());
            }

            Ok(Answer::new(result))
        } else {
            // todo: add some errors here
            Err(Error::new(0))
        }
    }

    fn get_server_ip_from_string(server: &str) -> SocketAddr {
        match server.parse() {
            Ok(socket) => socket,
            Err(_) => panic!(),
        }
    }

    // todo: don't hard code like that (the index 0)
    // maybe try some load balancing, or random
    fn get_default_server(&self) -> Option<SocketAddr> {
        if self.default_servers.len() == 0 {
            return None;
        }
        Some(self.default_servers[0])
    }
}

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use crate::api::{DnsResult, DnsError};
use crate::udp_controller::UdpController;
use crate::message::Message;

pub struct Core {
    udp_controller: UdpController,
}

impl Core {
    const DEFAULT_UDP_BUFFER: u16 = 1024;

    const DEFAULT_DNS_SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    const DEFAULT_DNS_SERVER_PORT: u16 = 53;

    pub fn new() -> Self {
        let interface = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 199)), 0
        );

        let server = SocketAddr::new (
            Self::DEFAULT_DNS_SERVER_IP,
            Self::DEFAULT_DNS_SERVER_PORT
        );

        Self {
            udp_controller: UdpController::new(
                Self::DEFAULT_UDP_BUFFER, interface, server)
        }
    }

    pub fn send_query(&self, domain: String) -> Result<DnsResult, DnsError> {
        let msg = Message::new(domain);

        let encoded_msg = msg.encode();

        let encoded_response = self.udp_controller.send_query(&encoded_msg);

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
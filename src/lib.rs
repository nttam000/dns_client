pub mod config;
mod core;
mod net;
mod parser;

use crate::core::core::Core;

#[derive(PartialEq)]
pub enum Protocol {
    Udp,
    Tcp,
}

pub struct Answer {
    answers: Vec<Vec<u8>>,
}

pub struct Error {
    error_code: u8,
}

impl Answer {
    pub fn new(answers: Vec<Vec<u8>>) -> Self {
        Self {
            answers: answers.clone(),
        }
    }

    pub fn get_answers(&self) -> &Vec<Vec<u8>> {
        &self.answers
    }
}

impl Error {
    pub fn new(error_code: u8) -> Self {
        Self { error_code }
    }

    pub fn get_error_code(&self) -> u8 {
        self.error_code
    }
}

pub fn query(domain: &str) -> Result<Answer, Error> {
    let core = Core::new();
    core.query(domain)
}

pub fn query_with_options(
    domain: &str,
    server_ip: &str,
    edns_enable: bool,
    protocol: Protocol,
    tcp_fallback: bool,
) -> Result<Answer, Error> {
    let core = Core::new();
    core.query_with_options(domain, &server_ip, edns_enable, protocol, tcp_fallback)
}

pub mod config;
mod core;
mod net;
mod parser;

use crate::core::core::Core;

pub struct DnsResult {
    answers: Vec<Vec<u8>>,
}

pub struct DnsError {
    error_code: u8,
}

impl DnsResult {
    pub fn new(answers: Vec<Vec<u8>>) -> Self {
        Self {
            answers: answers.clone(),
        }
    }

    pub fn get_answers(&self) -> &Vec<Vec<u8>> {
        &self.answers
    }
}

impl DnsError {
    pub fn new(error_code: u8) -> Self {
        Self { error_code }
    }

    pub fn get_error_code(&self) -> u8 {
        self.error_code
    }
}

pub fn query(domain: &str) -> Result<DnsResult, DnsError> {
    let core = Core::new();
    core.send_query(domain)
}

pub fn query_with_server(
    domain: &str,
    server_ip: &str,
) -> Result<DnsResult, DnsError> {
    let core = Core::new();
    core.send_query_with_server(domain, &server_ip)
}

mod parser;
mod net;
mod core;
pub mod config;

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
            answers: answers.clone()
        }
    }

    pub fn get_answers(&self) -> &Vec<Vec<u8>> {
        &self.answers
    }
}

impl DnsError {
    pub fn new(error_code: u8) -> Self {
        Self {
            error_code
        }
    }

    pub fn get_error_code(&self) -> u8 {
        self.error_code
    }
}

pub fn query(domain: &str) -> Result<DnsResult, DnsError> {
    let core = Core::new();
    core.send_query(domain)
}

pub fn query_with_server(domain: &str, server_ip: &str) -> Result<DnsResult, DnsError> {
    let core = Core::new();
    core.send_query_with_server(domain, &server_ip)
}

#[cfg(test)]
mod tests {
    #[test]
    fn query() {
        let query_result = crate::query("quora.com");
        // let query_result = dns_client::query_with_server("quora.com", "8.8.4.4:53");

        match query_result {
            Ok(value) => {
                for ip in value.get_answers() {
                    for (id, byte) in ip.iter().enumerate() {
                        print!("{}", byte);

                        if id < ip.len() - 1 {
                            print!(".");
                        }
                    }
                    println!();
                }
            }
            Err(error) => {
                println!("{}", error.get_error_code());
            }
        }
    }

    #[test]
    fn init() {
        use crate::config::config_handler::CONFIG;
        println!("{}", CONFIG.udp_buffer_size);
    }
}

use crate::core::Core;

#[derive(Debug)]
pub struct DnsResult {
    answers: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct DnsError {
    error_code: u8
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
}

pub fn query(domain: String) -> Result<DnsResult, DnsError> {
    let core = Core::new();
    core.send_query(domain)
}

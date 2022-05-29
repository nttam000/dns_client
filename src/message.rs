// Please refer RFC1035, section 4. MESSAGES and RFC6895
// todo: Request and Response should share a same trait, really?

use crate::question::Question;
use crate::header::{ Header, HeaderFlags };
use crate::resource_record::{ ResourceRecord, ResourceRecords };

#[derive(Debug)]
pub struct Message {
    header: Header,
    question: Question,
    answers: ResourceRecords,
    authorities: ResourceRecords,
    additionals: ResourceRecords,
}

impl Message {
    pub fn new(domain_name: String) -> Self {
        Self {
            header: Header::new(),
            question: Question::new(domain_name),
            answers: ResourceRecords::new(),
            authorities: ResourceRecords::new(),
            additionals: ResourceRecords::new(),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut encoded_header = self.header.encode();
        let mut encoded_questions = self.question.encode();
        let mut encoded_answers = self.answers.encode();
        let mut encoded_authorities = self.authorities.encode();
        let mut encoded_additionals = self.additionals.encode();

        result.append(&mut encoded_header);
        result.append(&mut encoded_questions);
        result.append(&mut encoded_answers);
        result.append(&mut encoded_authorities);
        result.append(&mut encoded_additionals);

        result
    }

    pub fn parse(msg: &[u8]) -> Self {
        let mut pos = 0;

        let (header, parsed_count) = Header::parse(msg, pos);
        pos += parsed_count as usize;

        let (question, parsed_count) = Question::parse(msg, pos);
        pos += parsed_count as usize;

        let mut answers = ResourceRecords::new();

        for _ in 0..header.get_an_count() {
            let (answer, parsed_count) = ResourceRecord::parse(msg, pos);
            pos += parsed_count as usize;

            answers.push(answer);
        }

        let mut authorities = ResourceRecords::new();
        for _ in 0..header.get_ns_count() {
            let (authority, parsed_count) = ResourceRecord::parse(msg, pos);
            pos += parsed_count as usize;

            authorities.push(authority);
        }

        let mut additionals = ResourceRecords::new();
        for _ in 0..header.get_ar_count() {
            let (additional, parsed_count) = ResourceRecord::parse(msg, pos);
            pos += parsed_count as usize;

            additionals.push(additional);
        }

        Self {
            header,
            question,
            answers,
            authorities,
            additionals,
        }
    }

    pub fn get_answers(&self) -> &ResourceRecords {
        &self.answers
    }

    fn generate_id() -> u16 {
        unimplemented!();
    }
}
use super::dns_types::{QType, QClass};
use super::dns_types;
use super::domain_name::DomainName;

pub struct ResourceRecords {
    resource_records: Vec<ResourceRecord>
}

pub struct ResourceRecord {
    q_name: DomainName,
    q_type: QType,
    q_class: QClass,
    ttl: u32,
    rd_length: u16,
    r_data: Vec<u8>,
}

impl ResourceRecords {
    pub fn new() -> Self{
        Self {
            resource_records: Vec::new()
        }
    }

    pub fn push(&mut self, resource_record: ResourceRecord) {
        self.resource_records.push(resource_record)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut rrs = Vec::new();
        for rr in &self.resource_records {
            let mut encoded_rr =  rr.encode();
            rrs.append(&mut encoded_rr);
        }
        rrs
    }

    pub fn get_resource_records(&self) -> &Vec<ResourceRecord> {
        &self.resource_records
    }
}

impl ResourceRecord {
    pub fn new() -> Self {
        Self {
            q_name: DomainName::new(""),
            q_type: QType::A,
            q_class: QClass::In,
            ttl: 0,
            rd_length: 0,
            r_data: Vec::new(),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut domain_name = self.q_name.encode();
        let mut type_and_class =
            dns_types::encode_type_and_class(&self.q_type, &self.q_class);

        result.append(&mut domain_name);
        result.append(&mut type_and_class);

        // ttl
        result.push(((self.ttl & 0b_1111_1111_0000_0000_0000_0000_0000_0000) >> 24) as u8);
        result.push(((self.ttl & 0b_0000_0000_1111_1111_0000_0000_0000_0000) >> 16) as u8);
        result.push(((self.ttl & 0b_0000_0000_0000_0000_1111_1111_0000_0000) >> 08) as u8);
        result.push(((self.ttl & 0b_0000_0000_0000_0000_0000_0000_1111_1111) >> 00) as u8);

        // rd_data
        result.push(((self.rd_length & 0b_1111_1111_0000_0000) >> 8) as u8);
        result.push(((self.rd_length & 0b_0000_0000_1111_1111) >> 0) as u8);

        // r_data
        for i in 0..self.rd_length {
            result.push(self.r_data[i as usize]);
        }

        result
    }

    pub fn parse(msg: &[u8], offset: usize) -> (Self, u16) {
        let mut result = Self::new();

        let mut pos = offset;

        let (domain_name, parsed_count) = DomainName::parse(msg, pos);
        pos += parsed_count as usize;
        result.q_name = domain_name;

        let (q_type, q_class, parsed_count) = dns_types::parse_type_and_class(msg, pos);
        pos += parsed_count as usize;
        result.q_type = q_type;
        result.q_class = q_class;

        // ttl
        result.ttl |= (msg[pos] as u32) << 24;
        pos += 1;
        result.ttl |= (msg[pos] as u32) << 16;
        pos += 1;
        result.ttl |= (msg[pos] as u32) << 8;
        pos += 1;
        result.ttl |= (msg[pos] as u32) << 0;
        pos += 1;

        // rd_length
        result.rd_length |= (msg[pos] as u16) << 8;
        pos += 1;
        result.rd_length |= (msg[pos] as u16) << 0;
        pos += 1;

        for _ in 0..result.rd_length {
            result.r_data.push(msg[pos]);
            pos += 1;
        }

        let total_parsed_count: u16= (pos - offset).try_into().
            expect("can not happen ever as msg length is controlled");

        (result, total_parsed_count)
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.r_data
    }
}
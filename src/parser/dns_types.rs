pub enum QType {
    A,
    Ns,
    CName,
    Aaaa,
    Opt,
    FutureUse,
}

pub enum QClass {
    In,
    Cs,
    Ch,
    Hs,
    FutureUse,
}

#[derive(PartialEq)] // todo: learn this
pub enum RCode {
    NoError,
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
    NotZone,
    FutureUse,
}

impl QType {
    pub fn get_value(&self) -> Option<u16> {
        match self {
            QType::A => Some(1),
            QType::Ns => Some(2),
            QType::CName => Some(5),
            QType::Aaaa => Some(28),
            QType::Opt => Some(41),
            _ => None,
        }
    }

    pub fn get_q_type(value: u16) -> Self {
        match value {
            1 => QType::A,
            2 => QType::Ns,
            5 => QType::CName,
            28 => QType::Aaaa,
            41 => QType::Opt,
            _ => QType::FutureUse,
        }
    }
}

impl QClass {
    pub fn get_value(&self) -> Option<u16> {
        match self {
            QClass::In => Some(1),
            QClass::Cs => Some(2),
            QClass::Ch => Some(3),
            QClass::Hs => Some(4),
            _ => None,
        }
    }

    pub fn get_q_class(value: u16) -> Self {
        match value {
            1 => QClass::In,
            2 => QClass::Cs,
            3 => QClass::Ch,
            4 => QClass::Hs,
            _ => QClass::FutureUse,
        }
    }
}

impl RCode {
    pub fn get_value(&self) -> Option<u8> {
        match self {
            RCode::NoError => Some(0),
            RCode::FormErr => Some(1),
            RCode::ServFail => Some(2),
            RCode::NXDomain => Some(3),
            RCode::NotImp => Some(4),
            RCode::Refused => Some(5),
            RCode::YXDomain => Some(6),
            RCode::YXRRSet => Some(7),
            RCode::NXRRSet => Some(8),
            RCode::NotAuth => Some(9),
            RCode::NotZone => Some(10),
            RCode::FutureUse => None,
        }
    }

    pub fn get_r_code(value: u8) -> Self {
        // r_code value has only 4 bits.
        assert!(value < 16);
        match value {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NXDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
            6 => RCode::YXDomain,
            7 => RCode::YXRRSet,
            8 => RCode::NXRRSet,
            9 => RCode::NotAuth,
            10 => RCode::NotZone,
            _ => RCode::FutureUse,
        }
    }
}

pub fn parse_type_and_class(msg: &[u8], offset: usize) -> (QType, QClass, u16) {
    let q_type = QType::get_q_type(
        (msg[offset + 0] as u16) << 8 | (msg[offset + 1] as u16),
    );

    let q_class = QClass::get_q_class(
        (msg[offset + 2] as u16) << 8 | (msg[offset + 3] as u16),
    );

    (q_type, q_class, 4)
}

pub fn encode_type_and_class(q_type: &QType, q_class: &QClass) -> Vec<u8> {
    let mut result = Vec::new();

    let q_type_value = q_type.get_value().unwrap();
    let q_class_value = q_class.get_value().unwrap();

    result.push((q_type_value >> 8) as u8);
    result.push((q_type_value >> 0) as u8);
    result.push((q_class_value >> 8) as u8);
    result.push((q_class_value >> 0) as u8);

    result
}

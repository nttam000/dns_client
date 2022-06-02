use toml::Value;
use toml::Value::Array;
use std::env;
use std::fs;
use lazy_static::lazy_static;

const CONFIG_FILE_PATH: &str = "config.toml";

#[derive(Debug)]
pub struct Config {
    pub udp_buffer_size: usize,
    pub protocol: String,
    pub tcp_fallback: bool,
    pub edns_enable: bool,
    pub local_interface: String,
    pub default_servers: Vec<String>
}

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}

// todo: use something called serilization
pub fn load_config() -> Config {
    let contents = fs::read_to_string(CONFIG_FILE_PATH)
        .expect("something wrong with reading file");

    let toml_value = contents.parse::<Value>().expect("invalid config file");

    let udp_buffer_size = match toml_value["udp_buffer_size"] {
        Value::Integer(value) => value,
        _ => { panic!("") }
    };

    let protocol = match &toml_value["protocol"] {
        Value::String(value) => value.clone(),
        _ => { panic!("") }
    };

    let tcp_fallback = match toml_value["tcp_fallback"] {
        Value::Boolean(value) => value,
        _ => { panic!("") }
    };

    let edns_enable = match toml_value["edns_enable"] {
        Value::Boolean(value) => value,
        _ => { panic!("") }
    };

    let local_interface = match &toml_value["local_interface"] {
        Value::String(value) => value.clone(),
        _ => { panic!("") }
    };

    let default_servers = match &toml_value["default_servers"] {
        Value::Array(vec) => {
            let mut ips: Vec<String> = Vec::new();
            for ip in vec {
                if let Value::String(ip_str) = ip {
                    ips.push(ip_str.clone());
                }
            }
            ips
        },
        _ => { panic!("") }
    };

    assert!(udp_buffer_size >= 512);
    let result = Config {
        udp_buffer_size: udp_buffer_size as usize,
        protocol,
        tcp_fallback,
        edns_enable,
        local_interface,
        default_servers
    };

    println!("{:?}", result);

    result
}
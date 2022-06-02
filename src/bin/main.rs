// example code
use dns_client;
use dns_client::config::toml_reader::CONFIG;

extern crate lazy_static;

fn demo_dns() {
    let query_result = dns_client::query("quora.com");
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
fn test_init() {
    println!("{}", CONFIG.udp_buffer_size);
}

fn main() {
    // demo_dns();
    // TomlReader::test();
    test_init();
}
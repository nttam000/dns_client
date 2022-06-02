// example code
use dns_client;

fn main() {
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
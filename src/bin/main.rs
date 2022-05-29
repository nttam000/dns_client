// example code
use dns_client;

fn main() {
    let query_result = dns_client::query(&String::from("quora.com"));
    match query_result {
        Ok(value) => {
            // println!("{:?}", value);
        }
        Err(error) => {
            // println!("{:?}", error);
        }
    }
}
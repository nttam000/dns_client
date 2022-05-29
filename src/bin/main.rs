use dns_client::api;

fn main() {
    // example code
    let query_result = api::query(String::from("quora.com"));
    match query_result {
        Ok(value) => {
            println!("{:?}", value);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}
use dns_client;

#[test]
fn query() {
    let query_result = dns_client::query("google.com");

    let answers = match query_result {
        Ok(value) => value.get_answers().clone(),
        Err(error) => {
            panic!("failed, error_code: {}", error.get_error_code())
        }
    };
    print_list_of_ips(&answers);
}

#[test]
fn query_with_server() {
    let query_result =
        dns_client::query_with_server("google.com", "8.8.4.4:53");

    let answers = match query_result {
        Ok(value) => value.get_answers().clone(),
        Err(error) => {
            panic!("failed, error_code: {}", error.get_error_code())
        }
    };
    print_list_of_ips(&answers);
}

fn print_list_of_ips(ips: &Vec<Vec<u8>>) {
    for ip in ips {
        for (id, byte) in ip.iter().enumerate() {
            print!("{}", byte);

            if id < ip.len() - 1 {
                print!(".");
            }
        }
        println!();
    }
}

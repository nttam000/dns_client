use dns_client::{query, query_with_options};
use dns_client::{Answer, Error, Protocol};

#[test]
fn test_query() {
    let query_result = query("google.com");
    analyze_query_result(&query_result);
}

#[test]
fn test_query_edns() {
    let query_result = query_with_options("google.com", "8.8.8.8:53", true, Protocol::Udp, false);
    analyze_query_result(&query_result);
}

#[test]
fn test_query_tcp() {
    let query_result = query_with_options("google.com", "8.8.8.8:53", false, Protocol::Tcp, false);

    analyze_query_result(&query_result);
}

#[test]
fn test_query_edns_tcp() {
    let query_result = query_with_options("google.com", "8.8.8.8:53", true, Protocol::Tcp, false);

    analyze_query_result(&query_result);
}

#[test]
fn test_query_fallback() {
    let query_result = query_with_options("google.com", "8.8.8.8:53", false, Protocol::Udp, true);

    analyze_query_result(&query_result);
}

#[test]
fn test_query_edns_fallback() {
    let query_result = query_with_options("google.com", "8.8.8.8:53", true, Protocol::Udp, true);

    analyze_query_result(&query_result);
}

fn analyze_query_result(result: &Result<Answer, Error>) {
    let answers = match result {
        Ok(value) => value.get_answers().clone(),
        Err(error) => {
            panic!("failed, error_code: {}", error.get_error_code())
        }
    };
    if answers.len() == 0 {
        // in practise, this should not be a fault
        panic!("failed, no answer");
    }
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

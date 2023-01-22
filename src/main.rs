use http_auth_basic::Credentials;

extern crate core;
use std::{env, process};
use std::time::Duration;
use tokio;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {

    //get arguments
    let args: Vec<String> = env::args().collect();
    // set argument 1
    let username = match args.get(1) {
        Some(un) => un,
        None => ""
    };

    // set argument 2
    let password = match args.get(2) {
        Some(pw) => pw,
        None => ""
    };

    // set argument 3
    let url = match args.get(3) {
        Some(u) => u,
        None => ""
    };

    // set argument 4
    let keyword = match args.get(4) {
        Some(k) => k,
        None => ""
    };


    //credentials
    let credentials = Credentials::new(username, password);
    let auth = credentials.as_http_header();

    if url == "" || keyword == "" {
        no_arg_error()
    } else {
        fetch(url, keyword, auth.as_ref()).await;
    }

}

// submit request
pub async fn fetch(url: &str, keyword: &str, auth_token: &str) {
    //set headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(http::header::AUTHORIZATION, auth_token.parse().unwrap());
    // let auth = AuthorizationHeader::try_from(&headers).unwrap();
    // ignore invalid certs since we may be checking self-signed
    let client = ClientBuilder::new()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(7))
        .build();

    let response = match client {
        Ok(r) => r.get(url).send().await,
        Err(_e) => process::exit(2)
    };

    let result_text = match response {
        Ok(r) => r.text().await,
        Err(_e) => process::exit(2)
    };

    //
    let to_print = result_text.as_ref();
    println!("{:?}", to_print);
    //

    match result_text.as_ref() {
        Ok(t) => match t.contains(keyword) {
            true => success(),
            false => failure()
        },
        Err(_e) => process::exit(2)
    };
}



fn success() {
    println!("{} {} {}", "connection successful", "|", "OK");
    process::exit(0)
}
fn failure() {
    println!("{} {} {}", "connection failed", "|", "CRITICAL");
    process::exit(2)
}
fn no_arg_error() {
    println!("{}", "Missing Arguments!!");
    println!("{}", "Format;");
    println!("{}", "./check_site **%username% **%password% %url% %keyword%");
    println!("{}", "~ All four arguments are required! ~ ");
    println!("{}", "** If site does not use basic auth, the values will be ignored.");
    process::exit(3);
}

extern crate serde_json;
use serde_json::json;

use reqwest::{Client, Response, StatusCode};

pub fn post(base_url: &String, data: &String) -> Response{
    let client = Client::new();
    let url = format!("{base}/{path}",
                      base = base_url,
                      path = "api/explorer/v1/transactions");

    let query = &json!({ "tx_body": data });
    print!("{}", url);

    print!("POST {}", url);

    let builder = client.post(&url);
    print!("Body: {}", serde_json::to_string_pretty(&query).unwrap());
    let builder = builder.json(&query);
    let response = builder.send().expect("Unable to send request");

    response
}

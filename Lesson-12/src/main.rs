
use reqwest::{Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug,Serialize, Deserialize)]
struct ResponseObject{
    pub args: HashMap<String, String>,
    pub headers: Headers,
    pub origin: String,
    pub url: String,
}

#[derive(Debug,Serialize, Deserialize)]
struct Headers {
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "X-Amzn-Trace-Id")]
    pub x_amzn_trace_id: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let body = reqwest::get(
        "https://httpbin.org/get"
    )
    .await?
    .text()
    .await?;

    let response: ResponseObject = serde_json::from_str(&body)?;

    println!("{:?}", response);
    println!{"Origin: {}", response.origin};
    println!{"URL: {}", response.url};
    println!{"Accept Header: {:?}", response.headers};

    Ok(())
}
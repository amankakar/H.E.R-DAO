
// use reqwest::{Error , client};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// #[derive(Debug,Serialize, Deserialize)]
// struct ResponseObject{
//     pub args: HashMap<String, String>,
//     pub headers: Headers,
//     pub origin: String,
//     pub url: String,
// }

// #[derive(Debug,Serialize, Deserialize)]
// struct Headers {
//     #[serde(rename = "Accept")]
//     pub accept: String,
//     #[serde(rename = "Host")]
//     pub host: String,
//     #[serde(rename = "X-Amzn-Trace-Id")]
//     pub x_amzn_trace_id: String,
// }
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let body = reqwest::get(
//         "https://httpbin.org/get"
//     )
//     .await?
//     .text()
//     .await?;

//     let response: ResponseObject = serde_json::from_str(&body)?;

//     println!("{:?}", response);
//     println!{"Origin: {}", response.origin};
//     println!{"URL: {}", response.url};
//     println!{"Accept Header: {:?}", response.headers};

//     Ok(())
// }


use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let client = Client::builder()
                .timeout(Duration::from_secs(10))// each request will have a timeout of 10 seconds,
                .build()?;
    let response = client
        .get("https://httpbin.org/get")
        .send();
    let response1 = client
        .get("https://httpbin.org/get")
        .send();
let (response , response1) = tokio::join!(response, response1);
match response {
    Ok(res) => println!("Response 1 :: {:?}", res),
    Err(e) => println!("Error in Response 1 :: {:?}", e)}
match response1 {
    Ok(res) => println!("Response 2 :: {:?}", res),
    Err(e) => println!("Error in Response 2 :: {:?}", e)
    }

    Ok(())
}

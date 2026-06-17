
use reqwest::{Error, Response};
#[tokio::main]
async fn main() -> Result<(), Error> {

    let response : Response = reqwest::get(
        "https://api.github.com"
    )
    .await?;

    println!("Status: {}", response.status());

    Ok(())
}
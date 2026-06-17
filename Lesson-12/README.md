# Async/Await, Futures, Tokio & Async HTTP Calls
Let First understand `Async`. By Async we means that the code will wait for the given result to return result then it will move to next instruction. the Rust achive this via Futures. So which mean we first need to understand the Futures.

## Futures :
A Future is a value that represents work that will complete later. if you have any background in javascript you can think of it like a Promise. where we assume that when this Promise is fulfilled then we move on.

```rust
async fn hello() -> String {
    "Hello".to_string()
}
```
This function will return :
```rust
Future<Output=String>
```
We will not get the result until we call this function as follow :

```rust
let msg= hello().await;
```
full code example using `tokio` crate.
```rust

async fn get_number() -> i32 {
    100
}

#[tokio::main]
async fn main() {
    let future = get_number();
    println!("Before awaiting the future.");
    let value = future.await; 
    println!("{}", value);
}

```
We need to add tokio to `Cargo.toml` file :
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```
Futures do not run by themselves. They need an executor.Tokio is the most popular async runtime in Rust. It schedules and executes asynchronous tasks. The `#[tokio::main]` macro creates and starts the runtime automatically.
Output :
```bash
Before awaiting the future.
100
```
## Async Vs Sync :
As we have already discuss earlier the in sync code the program will stop till he receive the response and will not perform other task like below example :
```rust
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Start");

    sleep(Duration::from_secs(5));

    println!("Done");
}
```
The Thread is block for 5 sec, it will do no thing just sleep after 5 sec it will do other task or move to next task.
But in case of Async like we have done in the first example its allows a function to return a Future, while await pauses execution until that Future completes.

## Task
Task is a lightweight unit of program managed by tokio. `tokio::spawn()` allows tasks to run independently and concurrently.
```rust
use tokio::task;

#[tokio::main]
async fn main() {

    let handle = task::spawn(async {
        println!("Task started");
        100
    });

    let result = handle.await.unwrap();

    println!("Result = {}", result);
}
```
Output :
```bash
Task started
Result = 100
```
## `join!`
The `join!` allow multiple async operation to run concurrently it waits for all task to complete concurrently
```rust
use tokio::time::{sleep, Duration};

async fn task1()->i32{
    println!("Task1 started");
    sleep(Duration::from_secs(2)).await;
    println!("Task1 completed");
    100

}
async fn task2()->i32{
    println!("Task2 started");
    sleep(Duration::from_secs(1)).await;
    println!("Task2 completed");
    200
}
#[tokio::main]
async fn main() {
    let (result1, result2) = tokio::join!(task1(), task2());
    println!("Result1 = {}", result1);
    println!("Result2 = {}", result2);
}
```
From the Output we can observe that both task are running concurrently  :
```bash
Task1 started
Task2 started
Task2 completed
Task1 completed
Result1 = 100
Result2 = 200
```
## Async Http Calls
Most of the rust application spend time waiting for external services such as:
- API
- Database
- RPC Nodes
- Web Servers

Async HTTP requests allow the program to perform other work while waiting for responses.
An Example of Http  call for this we need to install `reqwest` crate So our dependencies inside `Cargo.toml` will look like below :
```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```
```rust

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
```
Output :
```bash
Status: 403 Forbidden
```
## Processing API response
After receiving an HTTP response, we often need its content.

```rust

use reqwest::{Error};
#[tokio::main]
async fn main() -> Result<(), Error> {

    let body = reqwest::get(
        "https://httpbin.org/get"
    )
    .await?
    .text()
    .await?;

    println!("{}", body);

    Ok(())
}
```

Output : 
```bash
{
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Host": "httpbin.org", 
    "X-Amzn-Trace-Id": "Root=1-6a32e464-364286497488600153d0eece"
  }, 
  "origin": "203.101.190.61", 
  "url": "https://httpbin.org/get"
}
```
## Serde
As from our above example we can see that the API return a JSON response  but we have no way to parse it. To parse this response and process its content we need to use `Serde` crate. 

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
```
```rust

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
```
Output :
```bash
ResponseObject { args: {}, headers: Headers { accept: "*/*", host: "httpbin.org", x_amzn_trace_id: "Root=1-6a32eb9e-335f5f896235a42b6917a8f0" }, origin: "203.101.190.61", url: "https://httpbin.org/get" }
Origin: 203.101.190.61
URL: https://httpbin.org/get
Accept Header: Headers { accept: "*/*", host: "httpbin.org", x_amzn_trace_id: "Root=1-6a32eb9e-335f5f896235a42b6917a8f0" }
```



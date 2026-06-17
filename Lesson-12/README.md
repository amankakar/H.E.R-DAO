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

async fn hello() -> String {
    "Hello, world!".into()
}

#[tokio::main]
async fn main() {
    let message = hello().await;
    println!("{}", message);
}
```
We need to add tokio to `Cargo.toml` file :
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```
Output :
```bash
Hello, world!
```
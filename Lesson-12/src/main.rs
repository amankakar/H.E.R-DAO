
async fn hello() -> String {
    "Hello, world!".into()
}

#[tokio::main]
async fn main() {
    let message = hello().await;
    println!("{}", message);
}

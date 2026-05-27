use thiserror::Error;

#[derive(Debug, Error)]
enum ErrorMessage {
    #[error("UnderFlow: {0}")]
    UnderFlow(String),
    #[error("OverFlow: {0}")]
    OverFlow(String),
}

fn main() {
    let result = perform_sub();
    match result {
        Ok(value) => println!("Subtraction result: {:?}", value),
        Err(e) => println!("Error: {:?}", e),
    }
}

// this will Return the error
fn perform_sub() -> Result<u32, ErrorMessage> {
    let a: u32 = 6;
    let b: u32 = 10;

    match a.checked_sub(b) {
        Some(c) => Ok(c),
        None => Err(ErrorMessage::UnderFlow(format!(
            " {} - {} = UnderFlow",
            a, b
        ))),
    }
}

fn perform_add() -> Result<u32, ErrorMessage> {
    let a: u32 = 6;
    let b: u32 = 10;

    match a.checked_add(b) {
        Some(c) => Ok(c),
        None => Err(ErrorMessage::OverFlow(format!(" {} + {} = OverFlow", a, b))),
    }
}

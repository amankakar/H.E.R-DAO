# Topics Covered
## panic! vs recoverable errors

### panic!

A `panic!` occurs when the program encounters an unrecoverable error state. In such cases, Rust stops execution immediately and unwinds the stack.
```rust
fn main() {
    let a : u32 = 6 ;
    let b : u32 = 10;
    let c : u32 = a - b;
    println!("Itwill panic So this log will never get executed :  {} - {} = {}", a, b, c);
}
```
output :
```bash

thread 'main' (66298) panicked at src/main.rs:5:19:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### recoverable
A recoverable error is a situation where the program can handle the error and continue execution without crashing.

```rust
fn main() {
    let a : u32 = 6 ;
    let b : u32 = 10;

  match a.checked_sub(b) {
    Some(c) => println!(" {} - {} = {}", a, b, c),
    None => println!(" {} - {} = Underflow", a, b),
  }
}

```
output :
```bash
 6 - 10 = Underflow
```
In this case, the program does not panic. Instead, it handles the `None` case safely and continues execution.
## Result<T, E>

`Result<T, E>` is used when a function may either return a successful value `(Ok)` or an error `(Err)`.

It is the standard way in Rust to represent recoverable failures.
```rust
fn main() {
    let result = perform_sub();
    match result {
        Ok(value) => println!("Subtraction result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
fn perform_sub() -> Result<u32, String> {
    let a: u32 = 6;
    let b: u32 = 10;

    match a.checked_sub(b) {
        Some(c) => Ok(c),
        None => Err(format!(" {} - {} = Underflow", a, b)),
    }
}

```
output :

```bash
Error:  6 - 10 = Underflow
```

## ? operator
The `?` operator is used to propagate errors to the caller instead of handling them locally.

If an error occurs, it is automatically returned from the current function.

```rust
fn main() {
    let result = propegate_error();
    match result {
        Ok(value) => println!("Subtraction result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}


fn propegate_error() -> Result<u32, String> {
    let value = perform_sub()?; This function will propagate the error to the caller which means the caller needs to handle the error as well
    Ok(value)
}
// this will Return the error
fn perform_sub() -> Result<u32, String> {
    let a: u32 = 6;
    let b: u32 = 10;

    match a.checked_sub(b) {
        Some(c) => Ok(c),
        None => Err(format!(" {} - {} = Underflow", a, b)),
    }
}
```
output :

```bash
Error:  6 - 10 = Underflow
```
## custom error enums

Another way to handle errors in Rust is by defining custom error types using enums. This allows you to represent multiple error states in a structured way.

```rust

#[derive(Debug)]
enum ErrorMessage {
UnderFlow(String),
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
        None => Err(ErrorMessage::UnderFlow(format!(" {} - {} = UnderFlow", a, b))),
    }
}

fn perform_add()-> Result<u32, ErrorMessage> {
    let a: u32 = 6;
    let b: u32 = 10;

    match a.checked_add(b) {
        Some(c) => Ok(c),
        None => Err(ErrorMessage::OverFlow(format!(" {} + {} = OverFlow", a, b))),
    }
}

```

output :
```bash
Error: UnderFlow(" 6 - 10 = UnderFlow")
```
## thiserror

thiserror is the standard Rust crate used to create clean, idiomatic custom error types with very little boilerplate.

It automatically implements:
```rust 
Display
Debug
std::error::Error
```

```rust
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
```

output : 
```bash
Error: UnderFlow(" 6 - 10 = UnderFlow")
```
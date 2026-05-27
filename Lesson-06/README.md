# Topics Covered
## panic! vs recoverable errors

### panic!

If the Rust program could not recover or have no way to handle the error state it will emit the panic error: 

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
Recovereable is state where the rust program do not need to crash it can contune its normal executation.

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
The programm will not panic it will move the execution of None arm

## Result<T, E>
Result is usefull where we need to return either Error or acutal data . like in case if some error occured than we retuuuuurn the `Err` otherwise we return the `Ok`.

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
This `?` operator is used where we did not want to handle the error case instead we want to propegate it to other funciton.

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

The Other way to show the Error message and handle error case we can use enum to define different error state and then using match arm to emit the error with the appropraite enum. 

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
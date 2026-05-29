# Modules & Project Architecture
As Rust projects grow, keeping all the code inside a single main.rs file becomes difficult to manage. Rust provides a module system that helps organize code into multiple files and directories.

Modules improve:

- code readability
- maintainability
- reusability
- separation of concerns

## What is a Module?
A module is a namespace that contains:

- functions
- structs
- enums
- traits
- constants
- other modules

Modules help group related code together.

### Basic Module Example
```rust
mod math {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn main() {
    let result = math::add(5, 10);

    println!("Result: {}", result);
}
```
### Understanding `pub`
By default, everything inside a module is private.
```rust
fn add()
```
cannot be accessed outside the module.
To make it accessible, use:
```rust

pub fn add()
```

### Module File Structure
Rust modules can be split into separate files.

#### Example Project Structure
```rust
src/
├── main.rs
├── math.rs
```
inside `math.rs` file :
```rust
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```
Inside `main.rs` file :
```rust
mod math;

fn main() {
    let result = math::add(10, 20);

    println!("Result: {}", result);
}
```
#### Nested Modules
Modules can contain other modules.
##### Example Structure
```rust
src/
├── main.rs
├── blockchain/
│   ├── mod.rs
│   ├── block.rs
│   └── transaction.rs
```

- `blockchain/mod.rs`

    ```rust
    pub mod block;
    pub mod transaction;
    ```
- `blockchain/block.rs`
    ```rust
    pub struct Block {
    pub block_id: u64,
    }
    ```
- `blockchain/transaction.rs`
    ```rust
    pub struct Transaction {
    pub amount: u64,
    }
    ```
-  `main.rs`
    ```rust
    mod blockchain;

    use blockchain::block::Block;
    use blockchain::transaction::Transaction;

    fn main() {
        let block = Block { block_id: 1 };

        let tx = Transaction { amount: 100 };

        println!("{:?}", block.block_id);
        println!("{:?}", tx.amount);
    }
    ```

#### `use` Keyword
The use keyword imports items into the current scope.

Without `use`:
```rust
blockchain::block::Block
```
With `use`
```rust
use blockchain::block::Block
```
Now you can directly write:
```rust
Block
```
#### `crate::` Keyword
`crate::` refers to the root of the current crate.
##### Example:
```rust
use crate::blockchain::block::Block;
```
This is commonly used in larger projects.

#### `super::` Keyword
`super::` refers to the parent module.
##### Example
```rust
use super::transaction::Transaction;
```

### `lib.rs` vs `main.rs`

- `main.rs`
Used for executable applications.
Example:
```bash
cargo run
```
- `lib.rs`
Used for reusable libraries.

Example:
SDKs , crates , reusable logic



#### Visibility Rules in Rust
|Keyword	|Meaning|
|----|----|
|private|	Accessible only inside module|
|pub	|Public everywhere|
|pub(crate)	|Public within current crate|
|pub(super)	|Public to parent module|

### Re-exporting Modules
Rust allows re-exporting items.
#### Example
```rust
pub use block::Block;
```
Now users can import:
```rust 
use blockchain::Block;
```
instead of:
```rust
use blockchain::block::Block;
```
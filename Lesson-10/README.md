# Smart Pointers
Smart Pointer is data structure that behaves like a normal pointer but it also provide following additional properties :
- Ownership management
- Automatic memory cleanup
- Shared ownership
- Interior mutuality
- Thread safe sharing

In Rust smart pointer are usually implemented using following traits :
- **Deref :**   Allows a smart pointer to be treated like a regular reference, enabling the use of the dereference operator `(*)` to access the underlying value.
- **Drop:**  Automatically runs when a smart pointer goes out of scope to clean up heap data and free memory

Lets first see how normal  Reference works :
```rust
 let a = 4;
 let b = &a;
```
it only borrow data

But the Smart pointer like below :
```rust
let b = Box::new(4)
```
owns the data and also manage its lifetime.

## Why Rust Needs Smart Pointers
Smart pointer solve problems that normal pointer does not.

1. Heap Allocation of primitive data type :
By default rust store primitive data type at stack but smart pointer will store it at heap
```rust
Box<T>
```

2. Shared Ownership
It allows multiple owner of the same data
```rust
Rc<T>
Arc<T>
```

3. Interior Mutability
Allow mutation of data even if normal borrowing rule would prevent
```rust
RefCell<T>
Mutex<T>
RwLock<T>
```

## Type Of Smart Pointers

### Box<T>
The simplest smart pointer.

Stores data on the heap while the pointer itself lives on the stack.
```rust
fn main() {
    let a = Box::new(5); // The pointer will be stored on Stack while the value will be stored on Heap
    println!("a: {}", a);
}
```
```bash
Stack
+-------+
| Box   | ----+
+-------+     |
              v
Heap
+-------+
| 5   |
+-------+
```
Common Uses
Large data structures
Recursive types
Heap allocation

Example:

```rust

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}


fn main() {
    let list = Box::new(List::Cons(5, Box::new(List::Cons(4, Box::new(List::Cons(3, Box::new(List::Nil))))))); // The pointer will be stored on Stack while the value will be stored on Heap
    println!("{:#?}", list);
}

```
Output :
```bash
Cons(
    5,
    Cons(
        4,
        Cons(
            3,
            Nil,
        ),
    ),
)
```
Without Box, Rust would not know the size of List.

###  Rc<T> (Reference Counted)
This Smart pointer allows the multiple owner of same data.

```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(String::from("Hello"));
    {
        let a = Rc::clone(&value);
        let b = Rc::clone(&value);
        println!(
            "count when are reference are in scope {:?}",
            Rc::strong_count(&value)
        );
    }
    println!(" outside of the 2 ref scope {:?}", Rc::strong_count(&value));
}

```
Ownership map :
```bash
Rc Count = 3

value ─┐
a      ├──> "Hello"
b ─────┘
```
When the count reaches zero, the data is dropped. It is not thread Safe thats why it works only in a single thread.

Example :
```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(String::from("Hello"));
    {
        let a = Rc::clone(&value);
        let b = Rc::clone(&value);
        println!(
            "count when are reference are in scope {:?}",
            Rc::strong_count(&value)
        );
    }
    println!(" outside of the 2 ref scope {:?}", Rc::strong_count(&value));
}

```
Output :
```bash
count when are reference are in scope 3
outside of the 2 ref scope 1
```
###  Arc<T> (Atomic Reference Counted)
It works exactly as Rc<T> but it is thread safe .
```rust
    let value = Arc::new(String::from("Hello"));
```
Ownership map :
```bash
Thread 1
   |
   +---- Arc ----+
                 |
Thread 2         |
   |             |
   +-------------+
```
It provides shared configuration , shared counts and multithread usage of data.
Example :
```rust
use std::sync::Arc;

fn main() {
    let value = Arc::new(String::from("Hello"));

    let thread_value = Arc::clone(&value);
    let thread = std::thread::spawn(move || {
        let thread1_value = Arc::clone(&thread_value);
        println!("Thread ref count: {}", Arc::strong_count(&thread_value));
    });
    thread.join().unwrap();

    println!("Main ref count: {}", Arc::strong_count(&value));
}

```
Output :
```bash
Thread ref count: 3
Main ref count: 1
```

###  RefCell<T>
Provides interior mutability. Allows mutation through an immutable variable, The normal Rust check the borrow at compile time but `RefCell<T>` check it on runtime. So the Invalid borrow will cause panic.

Example: 
```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10); // value is mutable 

    *value.borrow_mut() += 5; // but we are still able to update its value

    println!("Value updated to {}", value.borrow());
}
```
Output : 
```bash
Value updated to 15
```
###  Weak<T>
Used with `Rc` or `Arc` to prevent reference cycles.
The problem with the `Rc` or `Arc` is that the reference count could never reach 0 like below :
```bash
A -> B
B -> A
```
Due to this the Memory leak occurs. So to avoid this rust introduces the Weak Pointer Because it does not increase the reference count.
```rust
Weak<T>
```
Example : 
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let a : i32 = 10;
    let weak_ref : RefCell<Weak<i32>> = RefCell::new(Weak::new());
println!(" weak ref count before creating strong ref {:?}", weak_ref.borrow().weak_count()); // zero count 
    
        let strong_ref : Rc<i32> = Rc::new(a); // create allocation for strong reference
        *weak_ref.borrow_mut() = Rc::downgrade(&strong_ref); //  Populate our Weak pointer by downgrading the strong data.
        let _a = Rc::clone(&strong_ref);

        println!(" weak ref count after creating strong ref {:?}", weak_ref.borrow().weak_count()); // one count
        println!(" weak ref count after creating strong ref {:?}", weak_ref.borrow().strong_count()); // 2 strong count
        // now let upgrade the weak point to get the strong reference
        if let Some(strong_ref_from_weak) = weak_ref.borrow().upgrade() {
            println!(" weak ref count after creating strong ref {:?}", weak_ref.borrow().strong_count()); // it will now print 3  strong count
        } 
}
```
Output :
```bash
weak ref count before creating strong ref 0
 weak ref count after creating strong ref 1
 weak ref count after creating strong ref 2
 weak ref count after creating strong ref 3
```
### Deref Trait
Smart pointers behave like references because they implement:
```rust
Deref
```
Example :
```rust
use std::ops::Deref;

fn main() {
let x = Box::new(5);

assert_eq!(5, *x);

}
```
Rust automatically dereferences smart pointers when needed.

| Smart Pointer    | Ownership     | Mutable? | Thread Safe?          | Runtime Check? | Main Use                             |
| ---------------- | ------------- | -------- | --------------------- | -------------- | ------------------------------------ |
| `Box<T>`         | Single        | Yes      | Yes (if `T` is)       | No             | Heap allocation                      |
| `Rc<T>`          | Multiple      | No       | ❌                     | No             | Shared ownership                     |
| `Arc<T>`         | Multiple      | No       | ✅                     | No             | Shared ownership across threads      |
| `Cell<T>`        | Single        | Yes      | ❌                     | No             | Interior mutability for `Copy` types |
| `RefCell<T>`     | Single        | Yes      | ❌                     | ✅              | Interior mutability                  |
| `Rc<RefCell<T>>` | Multiple      | Yes      | ❌                     | ✅              | Shared mutable state                 |
| `Mutex<T>`       | Single/Shared | Yes      | ✅                     | Lock-based     | Thread-safe mutation                 |
| `RwLock<T>`      | Single/Shared | Yes      | ✅                     | Lock-based     | Many readers, one writer             |
| `Weak<T>`        | Non-owning    | No       | Depends on `Rc`/`Arc` | No             | Prevent reference cycles             |




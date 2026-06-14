# Concurrency & Parallel Execution
## Concurrency
Concurrency means multiple tasks make progress during the same time period. Which means They may not execute simultaneously, but they can progress independently.
Example :
```bash
Task A
Task B
Task C
Task D
```
The tasks may share a single CPU core and take turns executing.
```bash
A -> B ->  -> A -> C -> B ->  C -> D -> A
```
## Parallel 
Parallelism means multiple tasks execute at the exact same time on different CPU cores.
Example :
```bash
Core 1 -> Task A
Core 2 -> Task B
Core 3 -> Task C
Core 4 -> Task D
```
## Rust Enable Concurrency and Parallelism Via following concept

### Threads 

A thread is the basic unit of execution. Which will run either parallel or concurrent with the other threads.
Example :
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // this create a new thread and runs the code inside the closure
        println!("Hello, world!");
    });
    handle.join().unwrap(); // this wait for the completion of the thread and returns the result of the thread, in this case it will return () because the closure does not return anything
}
```
Output :
```bash
Hello, world!
```
### Move Closures
From the above code you cna see that we did not move ownership of any variable from the main thread to the thread which we have created , what if we want to move the data to the thread from main thread for this we need to use the `move` keyword with out closure so that the variable from main thread can be moved to new thread

Example : 
```rust
use std::thread;

fn main() {
    let name = String::from("Aman");
    let handle = thread::spawn(move || { // transfer the ownership of name variable to thread
        println!("{}", name);
    });

    handle.join().unwrap();
}
```
Output :
```bash
Aman
```
**Note : Due to moving  the data to thread and if multiple thread modify data it results in race condition which rust stop at compile time.**
So for this issue the rust provide Arc<T> to use the shared ownership among threads which we have covered in our last lecture of Smart pointers.
**Arc** Example: 
```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let  name = Arc::new(String::from("Aman"));
    let cloned = Arc::clone(&name); // here we create clone of the Arc data and pass the cloned data to thread , So now this name has 2 reference pointer count
    let handle = thread::spawn(move || {
        println!("{}", cloned);
    });

    handle.join().unwrap();
}
```
What if you want to also pass the data and update it inside thread , than we need to use mutex , which will lock the memory first So that Only one thread can modify data at a time.There will be no race condition
**Mutex** Example : 
```rust
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
fn main() {
    let name = Arc::new(Mutex::new(String::from("Aman")));
    let cloned = Arc::clone(&name);
    let handle = thread::spawn(move || {
        cloned.lock().unwrap().push_str(" Khan");
    });

    handle.join().unwrap();
    println!("{}", name.lock().unwrap());
}

```
Output :
```bash
Aman Khan
```
### Channel 
In All of the above we have shared the data between threads , which we have discuss that the modification of data will be problematic So what if we want to pass the data instead of sharing this is where the channel came into picture.
```bash
Thread A → Message → Thread B
```
There are different type of channel. 
#### MPSC
MPSC stand for multiple producer and single consumer in this channel type multiple threads can  clone the data but a single consumer can pull the data.
**Concept:** Infinite capacity queue. Multiple senders, one receiver.
**Usage:** Gathering logs, metrics, or task results from multiple background worker threads into a single manager thread.

```rust
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    // producer 1
    thread::spawn(move || {
        let name = "Alice".to_string();
        tx.send(name).unwrap();
    });
    // producer 2
    thread::spawn(move || {
        let new_name = "Aman".to_string();
        tx1.send(new_name).unwrap();
    });
    // single conusmer to pull data from multiple producers
    let name = rx.recv().unwrap();
    let new_name = rx.recv().unwrap();

    println!("{}", name);
    println!("{}", new_name);
}

```
Output :
```bash
Aman
Alice
```
#### SPMC
SPMC Stands for Single Producer and multiple consumer. A single thread broadcasts tasks or data to multiple worker threads that compete for messages
**Concept:** A capacity-limited channel. If capacity is 0, it is a rendezvous channel where the sender blocks until a receiver is actively ready to take the message.
**Usage:** Enforcing strict backpressure to prevent a fast producer from flooding memory.
```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let shared_receiver = Arc::new(Mutex::new(rx)); // in this we have shared the receiver end of the channel among multiple threads
    let rx1 = Arc::clone(&shared_receiver); // clone the Arc to get another reference to the shared receiver

    // consumer 1
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));

        let message = {
            let lock = rx1.lock().unwrap();
            lock.try_recv()
        }; // lock will be droped here, allowing other threads to acquire the lock and receive messages

        if let Ok(msg) = message {
            println!("Worker first accepted task: {}", msg);
        } else {
            println!("Worker first failed to receive a task.");
        }
        // Simulate doing work
        // thread::sleep(Duration::from_millis(100));
    });
    // consumer 2
    let handle1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));

        let message = {
            let lock = shared_receiver.lock().unwrap();
            lock.try_recv()
        };
        if let Ok(msg) = message {
            println!("Worker second accepted task: {}", msg);
        } else {
            println!("Worker second failed to receive a task.");
        }
        // Simulate doing work
        // thread::sleep(Duration::from_millis(100));
    });
    let i = 1;
    tx.send(format!("Job #{}", i)).unwrap();

    drop(tx);

    let handles = vec![handle, handle1];
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All jobs completed successfully.");
}

```
Output :
```bash
Worker first accepted task: Job #1
Worker second failed to receive a task.
All jobs completed successfully.
```
From the above log it is clear that the worker are competitor so if first one consume the message the channel will be emapty and the next one will get no thing to avoid such condition we can send other message or use broadcast approach.

There are other types also but i am not going to cover it here for now ,



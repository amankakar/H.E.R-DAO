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

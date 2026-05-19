## Topic Covered In this Lesson
- ##  String vs &str :
    - String type is owned, Growable , Dynamic and  stored in Heap.
    ```rust 
    fn main() {
        let name = String::from("rust");
        println!("{}" , name);
    }
    ```
    - &str type is borrowed , fixed-sized , lightweight and  immutable
    ```rust
        let name = "rust";
        println!("{}" , name);
    ```
---
- ##  Vec<T>
    A vector `(Vec<T>)` is basically a dynamic list to store data. All the data types must be same
    ```rust
    fn main() {
        let numbers = vec![1, 2, 3];
        println!("{:?}", numbers);

        println!("{}", numbers[0]);
        /// using match to avoid the crash when we try to access an index that is out of bounds
        match numbers.get(1) {
            Some(number) => println!("{}", number),
            None => println!("No number found"),
        }
    }
    ```
---
- ##  HashMap
    HashMap is used to store the data like in vec but here the data got store in key-value pair format , where each data can be accessed using its key.
    ```rust
    use std::collections::HashMap;
    fn main() {
        let mut balances = HashMap::new();
        balances.insert("Alice", 100);
        balances.insert("Bob", 200);
        balances.insert("Charlie", 300);
        println!("{:?}", balances);

        println!("{}", balances[&"Alice"]);
        /// using match to avoid the crash when we try to access a key that does not exist
        match balances.get(&"Bob") {
            Some(number) => println!("{}", number),
            None => println!("No Data found"),
        }
        // in case we try to access data which is not in given map
        match balances.get(&"None") {
            Some(number) => println!("{}", number),
            None => println!("No Data found"),
        }
    }
    ```
---
- ##  Enums
    Enum is used to store the data when the data can be in one state among know list of other possible state i.e account it could be either active or not active.
    ```rust
    fn main() {
        enum Status {
            Active,
            Inactive,
            Pending(String), // we can also store the data in enum
        }
        let user_status = Status::Pending(String::from("Waiting for approval"));
        match user_status {
            Status::Active => println!("User is active"),
            Status::Inactive => println!("User is inactive"),
            Status::Pending(message) => println!("User is pending: {}", message),   
        }
    }
    ```
---
- ##  Option
    It used when we need to handle the case if the given data is not present. it is useful to avoid the crash of rust application.
    ```rust 
    fn main() {
        enum Option<T> {
            Some(T), // if data is present
            None, // if data is missing
        }
        let user_status = Option::Some(String::from("Waiting for approval"));
        match user_status {
            Option::Some(message) => println!("User is pending: {}", message),
            Option::None => println!("User status is unknown"), 
        }

        let new_status: Option<String> = Option::None; // is case of none, we need to specify the type of Option, otherwise it will be ambiguous
        match new_status {
            Option::Some(message) => println!("User is pending: {}", message),  
            Option::None => println!("User status is unknown"),
        }
    }
    ```
---
- ##  Result
    it is used to handled the Error case.
    ```rust
        fn main() {
        enum Result<T> {
            Ok(T), 
            Err, 
        }

        let a = 80;
        let b = 45;
        let result = divide(a, b);
        match result {
            Ok(value) => println!("Result of division: {}", value),
            Err(message) => println!("Error: {}", message),
        }
        // Handled the underflow case , where the denominator is zero
        let a = 43;
        let b = 0;
        let result = divide(a, b);
        match result {
            Ok(value) => println!("Result of division: {}", value),
            Err(message) => println!("Error: {}", message),
        }
    }

    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }
    ```
---
- ##  Pattern Matching
    It helps programs make decisions clearly.
    ```rust 
    fn main(){

        let number = 2;
        match number {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            _ => println!("Other"), // this is used to match any other value that is not 1, 2, or 3
        }
    }
    ```


# Iteration and Data Flow
Iteration is the process of traversing the collection of data like `vec!` , `hashMap` , `array` or iterators and performing the operation on its elements.
## `for` loop
The simplemst form of iteration is using for loop the rust behind the secens calls the `IntoIterator` function

```rust
for item in numbers{
    println!({} , item);
}
```

Before moving to the next topics of iteration we need to have a basic understanding of closure because they are heavily used with iterators.
## What is |x|?
|x| is a closure (an anonymous function).

Think of it as a function without a name.

For example:
```rust
fn double(x: i32) -> i32 {
    x * 2
}
```
can be written as a closure:

```rust
|x| x * 2
```
Multiple Parameters
```rust
|a, b| a + b

```
## `.iter()`
Creates an iterator that yields immutable references `(&T)` to elements without taking ownership.

```rust
fn main() {

let numbers =vec![1,2,3,4,5];
numbers.iter().for_each(|x| println!("{}", x));
}

```
## .iter_mut()
Creates an iterator that yields mutable references `(&mut T)`, allowing modification of elements.
```rust
fn main() {

let mut numbers =vec![1,2,3,4,5];
for number in numbers.iter_mut() {
    *number +=1 // iter_mut() returns a mutable reference to each element in the vector, allowing us to modify the values directly. By dereferencing the mutable reference with *, we can add 1 to each number in the vector.
}

println!("{:?}", numbers);
}

```
## .into_iter()
Consumes the collection and yields owned values `(T)`, transferring ownership.
```rust
fn main() {

let  numbers =vec![1,2,3,4,5];

let mut iter = numbers.into_iter();

assert_eq!(Some(1), iter.next());
assert_eq!(Some(2), iter.next());
assert_eq!(Some(3), iter.next());
assert_eq!(Some(4), iter.next());
assert_eq!(Some(5), iter.next());
assert_eq!(None, iter.next()); // full iter consumed
}
```

## .map()
Transforms each element into another value while preserving the number of elements.
```rust
fn main() {

let  numbers =vec![1,2,3,4,5];

let double = numbers.iter().map(|x| x * 2);

println!("{:?}", double.collect::<Vec<_>>());
}
```
Output :
```bash
[2, 4, 6, 8, 10]
```
iterators are lazy and do nothing unless consumed

let test this with our test code :
```rust
fn main() {

let  numbers =vec![1,2,3,4,5];

let double = numbers.iter().map(|x| x * 2);
println!("{:?}", double);
}
```
Output :
we expect the each element in our vector will be ultipled with 2 and the result will be stored in double var but the output show the exact same element not modification why , becuase we need to consume the iterator if we do not then it will not be executed that why i have called the collect method on iterator which will consumed the iterator and gives us the desired outout
```bash
Map { iter: Iter([1, 2, 3, 4, 5]) }
```

##  .filter()
Keeps only elements that satisfy a condition.

```rust
use std::vec;

fn main() {

let  numbers =vec![1,2,3,4,5];

let double = numbers.iter().filter(|x| *x % 2 ==0);
println!("{:?}", double.collect::<Vec<_>>());
}

```

Output :
```bash
[2, 4]
```

##  .filter_map()
Combines filtering and transformation in a single operation.
```rust
fn main() {
    let numbers: Vec<String> = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
    ];

    let dd = numbers
        .iter()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    println!("{:?}", dd);
}

```
Output :
```bash
[1, 2, 3, 4, 5]
```
## .find()
Returns the first element that matches a condition.
```rust
use std::vec;

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let found = numbers.iter().find(|x| x > &&3); // x is &&i32 because iter() yields references; compare to &&3 so the types match
    match found {
        Some(value) => println!("Found a number greater than 3: {}", value),
        None => println!("No number greater than 3 found."),
    }
}

```
output :
```bash
Found a number greater than 3: 4
```
## .position()
Return the index of first element matching the condition
```rust
fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let index = numbers.iter().position(|x| *x == 3); // x is &&i32 because iter() yields references; compare to &&3 so the types match
    match index {
        Some(value) => println!("Found the index of 3: {}", value),
        None => println!("3 not found."),
    }
}

```
Output :
```bash
Found the index of 3: 2
```
## .any()
It will return `true` at least one element satisfy the condition

```rust
fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let found = numbers.iter().any(|x| *x == 3); // here we need to de reference the value to compare it with 3 because rust does not provide impl of == with the reference type
    println!("Found 3: {}", found);
}

```
Output :
```bash
Found 3: true
```
## .all()
It will return true if all the element satisfy the condition
```rust
fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let found = numbers.iter().all(|x| *x > 0); // here we need to de reference the value to compare it with 3 because rust does not provide impl of == with the reference type
    println!("All numbers are positive: {}", found);
}
```
##  .fold()
Reduces an iterator into a single value using an accumulator.
```rust
fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; 

    let accumulate = numbers.iter().map(|x| x * 2).fold(0, |acc, x| acc + x );  // it will reduce the result to one value and provide the sum of all values in iterator
    println!("Sum of all numbers: {}", accumulate);
}
```
Output :
```bash
Sum of all numbers: 30
```
## chain
We can chain the multiple function of iterator in s single statement
```rust
fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let accumulate = numbers
        .iter()
        .filter(|x| *x % 2 == 0) // it will filter out the even numbers from the iterator
        .map(|x| x * 2) // it will multiply each value in the iterator by 2
        .fold(0, |acc, x| acc + x); // it will reduce the result to one value and provide the sum of all values in iterator
    println!("Sum of all numbers: {}", accumulate);
}

```
Output : 

```bash
Sum of all numbers: 12
```
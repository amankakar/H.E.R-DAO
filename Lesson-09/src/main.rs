fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let accumulate = numbers
        .iter()
        .filter(|x| *x % 2 == 0) // it will filter out the even numbers from the iterator
        .map(|x| x * 2) // it will multiply each value in the iterator by 2
        .fold(0, |acc, x| acc + x); // it will reduce the result to one value and provide the sum of all values in iterator
    println!("Sum of all numbers: {}", accumulate);
}

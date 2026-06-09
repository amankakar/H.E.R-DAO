use std::ops::Deref;

fn main() {
let x = Box::new(5);

assert_eq!(5, *x);

}
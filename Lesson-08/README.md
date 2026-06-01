# Traits and Abstraction
Traits are one of Rust's most powerful features. They allow you to define shared behavior across different types without requiring inheritance.
Rust traits are more powerful because they support:

- Default implementations
- Generic constraints
- Static dispatch
- Dynamic dispatch
- Associated types
- Operator overloading
- Trait objects

### Why Do We Need Traits?
Imagine we have different animals:
```rust 
struct Dog;
struct Cat;
struct Bird;
```
Each animal can make a sound.
Without traits:
```rust
impl Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
```
**How can we write a function that accepts any animal?**
Traits solve this problem.
### Defining a Trait
```rust
trait Animal {
    fn speak(&self);
}
```

This defines a contract:
> Any type implementing Animal must provide a `speak()` method.
### Implementing a Trait
Let create a New Struct `Dog` and `Cat` and impl Animal trait for them.
```rust
struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

```
### Using Traits
How to use/call speak method:
```rust
fn main() {
    let dog = Dog;
    let cat = Cat;
    dog.speak();
    cat.speak();
}
```

Output :
```bash
Woof!
Meow!
```
## Traits as Function Parameters
Instead of accepting a specific type:
```rust
fn make_sound(dog : Dog) {}
```
Accept any type implementing Animal

```rust
fn make_sound(animal : impl Animal) {}
```

How our new code will look like :
```rust 

fn make_sound(animal : impl Animal) {
    animal.speak();
}
```

Usage according to new code :
```rust
fn main() {
    let dog = Dog;
    let cat = Cat;
    make_sound(dog);
    make_sound(cat);
}
```

### Generic Trait Bounds
The previous syntax is shorthand for:
```rust

fn make_sound <T: Animal>(animal: T) {
    animal.speak();
}
```

### Multiple Trait Bounds
An Struct can impl multiple traits :
```rust
trait Animal {
    fn speak(&self);
}

trait Walkable {
    fn walk(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

impl Walkable for Dog {
    fn walk(&self) {
        println!("The dog is walking.");
    }
}
impl Walkable for Cat {
    fn walk(&self) {
        println!("The cat is walking.");
    }
}

fn interact_with_animal <T: Animal + Walkable>(animal: T) {
    animal.speak();
    animal.walk();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    interact_with_animal(dog);
    interact_with_animal(cat);
}

```

Output :

```bash
Woof!
The dog is walking.
Meow!
The cat is walking.
```

#### Default Implementations
Traits can provide default behavior. Now lets update our current code to test it.
The dog override default impl and the cat does not:

```rust
trait Animal {
    fn speak(&self){
        println!("The animal makes a sound.");
    }
}

trait Walkable {
    fn walk(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {}

impl Walkable for Dog {
    fn walk(&self) {
        println!("The dog is walking.");
    }
}
impl Walkable for Cat {
    fn walk(&self) {
        println!("The cat is walking.");
    }
}

fn interact_with_animal <T: Animal + Walkable>(animal: T) {
    animal.speak();
    animal.walk();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    interact_with_animal(dog);
    interact_with_animal(cat);
}

```

Output :
```rust
Woof!
The dog is walking.
The animal makes a sound.
The cat is walking.
```

### Returning Types that Implement Traits
Instead of:
```rust
fn create_dog() -> Dog
```
we can just return the trait type . 

```rust
fn create_dog () -> impl Animal 
```
The caller only knows:

> I get something that behaves like an Animal.

Our need code now look like :
```rust
trait Animal {
    fn speak(&self){
        println!("The animal makes a sound.");
    }
}

trait Walkable {
    fn walk(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {}

impl Walkable for Dog {
    fn walk(&self) {
        println!("The dog is walking.");
    }
}
impl Walkable for Cat {
    fn walk(&self) {
        println!("The cat is walking.");
    }
}

fn interact_with_animal <T: Animal + Walkable>(animal: T) {
    animal.speak();
    animal.walk();
}

fn create_dog() -> impl Animal + Walkable {
    Dog
}

fn create_cat() -> impl Animal + Walkable {
    Cat
}
fn main() {
    let dog = create_dog();
    let cat = create_cat();
    interact_with_animal(dog);
    interact_with_animal(cat);
}

```

Output :
```bash
Woof!
The dog is walking.
The animal makes a sound.
The cat is walking.
```
## Trait Objects (Dynamic Dispatch)
Suppose we want:
```rust 
let animals = vec![Cat , Dog]
```
It will not work because the `Cat` and `Dog` both have different type but the `vec!` only work with the same type.

So To fix this issue we need some thing call dynamic dispatch . So by dynamic we mean that we only load the value not the type at compile time So at the run time we load the type of value.

```rust
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];
```
So now the full code for our example will look like :

```rust
trait Animal {
    fn speak(&self) {
        println!("The animal makes a sound.");
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {}

fn interact_with_animal(animal: Box<dyn Animal>) {
    animal.speak();
}

fn create_dog() -> impl Animal {
    Dog
}

fn create_cat() -> impl Animal {
    Cat
}
fn main() {
    let dog = create_dog();
    let cat = create_cat();
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    for animal in animals {
        interact_with_animal(animal);
    }
}

```

Output : 
```bash
Woof!
The animal makes a sound.
```

###  Trait Objects and Object Safety
Not every trait can become a trait object.
This works:
```rust
trait Animal {
    fn speak(&self);
}
```
This doesn't
```rust
trait Animal {
    fn create() -> Self;
}
```
Because self is not object-safe. and also Object safety becomes important when using dynamic dispatch:

```rust
Box<dyn Animal>
```

|Static Dispatch |Dynamic Dispatch|
|----|----|
|Compiler generates code at compile time.|Compiler generates code at run time.|
|Faster|Slower|
|No run time cost|More flexible|
|monomorphization|Heterogeneous collections|

## Trait Inheritance
Traits can require other traits.
```rust
trait Pet : Animal {
    fn play(&self) {
        println!("The pet is playing.");
    }
}
```
Any type implementing Pet must also implement Animal.

So here we need to provide impl `Pet` for `Dog`
```rust
impl Pet for Dog {}

// Add the function which will call plat method on Animal
fn paly_with_pet(pet: Box<dyn Pet>) {
    pet.play();
}

// now inside main function we will call paly_with_pet for Dog
    paly_with_pet(Box::new(Dog)); // This will work because `Dog` implements `Pet`.

```
The output of our code is :
```bash
Woof!
The animal makes a sound.
The pet is playing.
```

When to Use Traits

Use traits when:

✅ Multiple types share behavior

✅ You want abstraction

✅ You want generic code

✅ You want dependency injection

✅ You want pluggable components
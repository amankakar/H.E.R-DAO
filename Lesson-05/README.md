# Lesson-05 :  Structs & State Design in Rust

## Struct :
Struct is custom data type which is used to define custom type by using primitives types or other custom types.

- #### Struct Definition 
```rust
struct User {
        username: String,
        email: String,
        active: bool,
    }
```
- #### Struct Instance and Usage
    Inside Main function we need to create the instance of struct and assign data to it like below
```rust
fn main() {
    let user1 = build_user(String::from("amanKhan@gmail.com"), String::from("Aman Khan"));
    println!("User Name: {} ", user1.username);
    println!("User Email: {} ", user1.email);
    println!("User Active: {} ", user1.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email : email,
        username : username,
        active: true,
    }
}
```
output :
```shell
User Name: Aman 
User Email: amankhan@gmail.com 
User Active: true 
```

- #### Using shorthand 
Using shortHand we do not need to using the struct Filed name while assign data to it.
```rust 
fn build_user(email: String, username: String) -> User {
    User {
         email,
         username,
        active: true,
    }
}
```

- #### Updating Data of Struct
First thing first we need to add the function which will update the provided instance of struct. 

```rust 
fn update_user(user: &mut User, email: String, username: String) {
    user.email = email;
    user.username = username;
}
```
Now inside our main function we are going to update the our already define `user1` data like below :

```rust 
fn main() {
    let mut user1 = build_user(String::from("amanKhan@gmail.com"), String::from("Aman Khan"));
    println!("User Name: {} ", user1.username);
    println!("User Email: {} ", user1.email);
    println!("User Active: {} ", user1.active);

    update_user(&mut user1, String::from("updated_email@gmail.com"), String::from("Updated Name"));
    println!("Updated User Name: {} ", user1.username);
    println!("Updated User Email: {} ", user1.email);
}

```

output :
```shell
User Name: Aman Khan 
User Email: amanKhan@gmail.com 
User Active: true 
Updated User Name: Updated Name 
Updated User Email: updated_email@gmail.com 
```

- #### Debug and Print Struct
Rust does not allow us to print complete struct by default. 

```rust 
struct User {
        username: String,
        email: String,
        active: bool,
    }
    

fn main() {
    let  user1 = build_user(String::from("amanKhan@gmail.com"), String::from("Aman Khan"));
    println!("{:?}" , user1);

}
fn build_user(email: String, username: String) -> User {
    User {
         email,
         username,
        active: true,
    }
}

```

output :
```shell
error[E0277]: `User` doesn't implement `Debug`
  --> src/main.rs:13:19
   |
13 | println!("{:?}" , user1);
   |           ----    ^^^^^ `User` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |           |
   |           required by this formatting parameter
   |
   = help: the trait `Debug` is not implemented for `User`
   = note: add `#[derive(Debug)]` to `User` or manually `impl Debug for User`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `User` with `#[derive(Debug)]`
   |
 1 + #[derive(Debug)]
 2 | struct User {
   |
```

To print the struct data we need to add attribute `Debug` to our struct definition the  Rust automatically generates trait implementation code for us. 

The Fix for this error is :
```rust 
#[derive(Debug)]
struct User {
        username: String,
        email: String,
        active: bool,
    }
    

fn main() {
    let  user1 = build_user(String::from("amanKhan@gmail.com"), String::from("Aman Khan"));
    println!("{:?}" , user1);

}
fn build_user(email: String, username: String) -> User {
    User {
         email,
         username,
        active: true,
    }
}

```
output :
```rust 
User { username: "Aman Khan", email: "amanKhan@gmail.com", active: true }
```
- #### Borrowing and Ownership in Structs

The concepts of borrowing and ownership apply to structs in the same way they apply to normal variables. Let’s take an example using our `User` struct.

The `User` struct contains a `username` field of type `String`. As we know, a `String` stores its actual data on the heap, while a reference to that data is stored on the stack. Because of this, ownership of a `String` can be moved or borrowed.

On the other hand, the `active` field is of type `bool`. A `bool` is stored entirely on the stack and implements the `Copy` trait. This means its value is copied instead of moved when assigned to another struct.

In the following code snippet, let’s see how ownership behaves differently for `String` and `bool` types.

```rust
fn main() {
    let user1 = build_user(
        String::from("amanKhan@gmail.com"),
        String::from("Aman Khan"),
    );

    println!("User Name: {}", user1.username);
    println!("User Email: {}", user1.email);
    println!("User Active: {}", user1.active);

    let user2 = User {
        email: String::from("Bob"),
        ..user1
    };

    println!("User Name: {}", user2.username);
    println!("User Email: {}", user2.email);
    println!("User Active: {}", user2.active);

    println!("User1 Email: {}", user1.email);
    println!("User1 Name: {}", user1.username);
    println!("User1 Active: {}", user1.active);
}
```

When we run this code using `cargo run`, Rust gives the following error:

```shell
error[E0382]: borrow of moved value: `user1.username`
  --> src/main.rs:22:33
   |
14 |       let user2 = User {
   |  _________________-
15 | |         email: String::from("Bob"),
16 | |         ..user1
17 | |     };
   | |_____- value moved here
...
22 |       println!("User1 Name: {} ", user1.username);
   |                                   ^^^^^^^^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
```

Can you guess what this error means?

As discussed earlier, the `username` field is of type `String`, which stores its data on the heap. When we create `user2` using struct update syntax (`..user1`), ownership of fields that do not implement the `Copy` trait is moved from `user1` to `user2`.

This means:
- `user1.username` is moved into `user2.username`

After the move, `user1` no longer owns these fields, so trying to access them causes a compile-time error.

However, the `active` field is of type `bool`, which implements the `Copy` trait. Instead of moving ownership, Rust creates a copy of the value for `user2`.

So:
- `user1.active` still remains valid
- `user2.active` gets its own copied value

This is why accessing `user1.active` works, while accessing `user1.username`  does not.
---

## Tuple
A fixed-size collection of values that can contain different types. no named filed , data can be access like in array and best for temporary storage and simple grouping.

```rust 
fn main() {

    let User = (String::from("amankhan@gmail.com") , String::from("Amankhan") , true);
    println!("User Email: {} ", User.0);
    println!("User Name: {} ", User.1);
    println!("User Active: {} ", User.2);
}

```
output :
```shell
User Email: amankhan@gmail.com 
User Name: Amankhan 
User Active: true 

```

Tuple are highly used for temprory storage of data like below :

```rust 
fn main() {
    let User: (String, String, bool) = get_user();
    println!("User Email: {} ", User.0);
    println!("User Name: {} ", User.1);
    println!("User Active: {} ", User.2);

}

fn get_user () -> (String, String, bool) {
    (String::from("amankhan@gmail.com") , String::from("Amankhan") , true)
}

```
output :
```shell
User Email: amankhan@gmail.com 
User Name: Amankhan 
User Active: true 
```

---

## Associated Functions
Associated Functions is a function which does not take &self as an argument , which means that it is not associated with the instance but with the type. All the function which we have defined above are associated functions:

```rust 
fn build_user(email: String, username: String) -> User {
    User {
         email,
         username,
        active: true,
    }
}
```
`build_user` is an associated function because it is not associated with the instance of user struct. We can also use or define associated function inside `impl` block. more on `impl` in next section

---

## impl block
`impl` is used to define methods and functions for struct and enum type. The methods use self and the functions do not use self.

```rust 
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let mut user1 = User::build_user(
        String::from("amanKhan@gmail.com"),
        String::from("Aman Khan"),
    );
    user1.print();
    user1.change_status(false);
    user1.print();
}

impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
        }
    }

    fn print(&self) {
        println!("{:?} ", self);
    }
    fn change_status(&mut self, status: bool) {
        self.active = status;
    }
}
```
output :
```shell
User { username: "Aman Khan", email: "amanKhan@gmail.com", active: true } 
User { username: "Aman Khan", email: "amanKhan@gmail.com", active: false } 
```
The `build_user` is a associated function. The `print` and `change_status` is methods.

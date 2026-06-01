fn main() {

    let mut report = String::from("Hello !");
    // Task 1: Add `fn append_signature(report: &mut String)` so callers can extend `report` without moving ownership.

    append_signature(&mut report);
    println!("This will print the updated string {}", report);

    // Task 2: Try creating two mutable references to the same `String` — what error do you get?
    //
    // Example that fails to compile:
    //   let mut report1 = &mut report;
    //   let mut report2 = &mut report; // second mutable borrow — error
    //
    // Typical compiler output:
    //   error[E0499]: cannot borrow `report` as mutable more than once at a time
    //    --> src/main.rs:NN:NN
    //     |
    //  NN |      let mut report1 = &mut report;
    //     |                          --------- first mutable borrow occurs here
    //  NN |      let mut report2 = &mut report;
    //     |                          ^^^^^^^^^ second mutable borrow occurs here
    //
    // Same binding names below; roles differ from the commented sketch: `report1` owns the buffer and `report2` is the sole `&mut` to it (not two `&mut report`).
    let mut report1 = String::from("Second report for 2 mut ref case");
    let mut report2 = &mut report1;

    // Task 3: Pass the `String` by value into a function and observe ownership.
    // The callee takes ownership of the heap data; returning `String` moves ownership back to the caller.
    let report = pass_by_value(report);
    println!("Now pass by value {}", report);

    // Task 4: Returning `&str` or `&String` that points at a local variable inside the function.
    // Locals are dropped when the function returns, so such a reference would dangle; Rust rejects this (often E0106 until lifetimes are spelled out — but lifetimes cannot make invalid borrows legal).
    // Safe alternatives include returning owned data (`String`), `'static` references, or tying the returned reference to an input lifetime.
    //
    // Broken sketch (does not compile):
    // fn fnc_local_str_ref() -> &String {
    //     let lc_var = String::from("Local_ref");
    //     &lc_var
    // }
    // let try_access = fnc_local_str_ref();
    // println!("{}", try_access);
}

fn append_signature(report: &mut String) {
    report.push_str("\n --Aman");
}

fn pass_by_value(mut report: String) -> String {
    report.push_str("\n --Now pass by value");
    report
}

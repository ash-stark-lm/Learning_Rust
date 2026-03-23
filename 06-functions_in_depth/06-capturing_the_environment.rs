/*
Closures can capture variables from the environment in three ways:

1. By immutable reference (&T)
   - The closure only reads the variable
   - Multiple immutable references are allowed

2. By mutable reference (&mut T)
   - The closure modifies the variable
   - Only one mutable reference is allowed at a time

3. By taking ownership (T)
   - The closure takes ownership of the variable
   - The original variable can no longer be used afterward
   - Happens for types that do NOT implement the Copy trait
     (like String, Vec, etc.)
*/

fn main() {
    /*
    Example 1: Capture by immutable reference (&T)

    The closure only reads the variable `x`,
    so Rust captures it by immutable reference.
    */

    let x = 10;

    let immutable_closure = || {
        println!("Immutable reference value: {}", x);
    };

    immutable_closure();

    // x is still valid because it was only immutably borrowed
    println!("After immutable_closure: {}", x);

    /*
    Example 2: Capture by mutable reference (&mut T)

    The closure modifies `counter`,
    so Rust captures it by mutable reference.
    */

    let mut counter = 0;

    let mut mutable_closure = || {
        counter += 5;
        println!("Inside mutable_closure: {}", counter);
    };

    mutable_closure();

    // counter is still valid because the closure only borrowed it mutably
    println!("After mutable_closure: {}", counter);

    /*
    Example 3: Capture by taking ownership (T)

    The closure takes ownership of `text`.
    This happens because we use the `move` keyword.
    */

    let text = String::from("hello");

    let ownership_closure = move || {
        let mut s = text;
        s.push_str(" world");
        println!("Inside ownership_closure: {}", s);
    };

    ownership_closure();

    // ❌ This would fail because `text` was moved into the closure
    // println!("{}", text);

    /*
    Example 4: Ownership with Copy types (i32)

    i32 implements the Copy trait, so even if captured by value,
    Rust copies it instead of moving it.
    */

    let number = 20;

    let copy_closure = move || {
        println!("Inside copy_closure: {}", number);
    };

    copy_closure();

    // number is still valid because i32 is Copy
    println!("After copy_closure: {}", number);
}

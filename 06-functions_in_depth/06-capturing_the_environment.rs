/*
Closures can capture variables from the environment in three ways:

1. By immutable reference (&T)
   - The closure only reads the value.
   - Multiple immutable references are allowed.

2. By mutable reference (&mut T)
   - The closure modifies the value.
   - Only one mutable reference is allowed at a time.

3. By taking ownership (T)
   - The closure takes ownership of the variable.
   - The original variable can no longer be used after that.
   - This happens for types that DO NOT implement the Copy trait
     (like String, Vec, etc.).
*/

/*
Immutable reference example

The function receives a shared reference (&i32).
It can read the value but cannot modify it.
*/
fn immutable_ref(x: &i32) {
    // *x += 1; ❌ Not allowed because x is immutable
    println!("Immutable reference value: {}", x);
}

/*
Mutable reference example

The function receives a mutable reference (&mut i32).
This allows the function to modify the original variable.
*/
fn mutable_ref(x: &mut i32) {
    *x += 5; // Dereference and modify the value
    println!("Inside mutable_ref: {}", x);
}

/*
Ownership example with String

The function takes ownership of the String.
After calling this function, the original variable
cannot be used because ownership has moved.
*/
fn takes_ownership(mut s: String) {
    s.push_str(" world"); // Modify owned data
    println!("Inside takes_ownership: {}", s);
}

/*
Ownership example with i32

Although the function parameter takes ownership,
i32 implements the Copy trait.

This means the value is copied instead of moved,
so the original variable is still valid.
*/
fn takes_ownership_int(x: i32) {
    println!("Inside takes_ownership_int: {}", x);
}

fn main() {
    /*
    Example 1: Immutable reference
    */
    let x: i32 = 10;

    immutable_ref(&x);

    // x is still valid because we only borrowed it
    println!("After immutable_ref: {}", x);

    /*
    Example 2: Mutable reference
    */
    let mut counter = 0;

    mutable_ref(&mut counter);

    // counter is still valid because the function only borrowed it
    println!("After mutable_ref: {}", counter);

    /*
    Example 3: Ownership transfer with String
    */
    let text = String::from("hello");

    takes_ownership(text);

    // ❌ This would fail because ownership moved to the function
    // println!("{}", text);

    /*
    Example 4: Ownership transfer with i32
    */
    let number = 20;

    takes_ownership_int(number);

    /*
    number is STILL valid here because i32 implements the Copy trait.
    Instead of moving ownership, Rust simply copies the value.
    */
    println!("After takes_ownership_int: {}", number);
}

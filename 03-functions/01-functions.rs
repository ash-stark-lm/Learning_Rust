/*
Functions in Rust are defined using the `fn` keyword,
followed by:
- Function name
- Parameters inside parentheses
- Return type (optional)
- Function body inside curly braces {}

A function can:
- Take zero or more parameters
- Return a value (or not)
*/

/*
⚠ Not Recommended (in this case)

fn greet(name: String) {
    println!("Hello, {}!", name);
}

Why?

Because this version takes ownership of the String.
After calling this function, the caller loses access to the variable.

Use `String` only when:
- You need to modify it
- You need to store it
- You need ownership
*/

// Recommended version
// `&str` is a borrowed string slice (read-only reference).
// The function does NOT take ownership.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with parameters and return type
// `-> i32` specifies the return type.
fn add(a: i32, b: i32) -> i32 {
    // In Rust, the last expression without a semicolon
    // becomes the return value.
    a + b
}

fn main() {
    println!("Hello, Functions!");

    // Calling greet with a string literal (&str)
    greet("John");

    // Calling add and storing returned value
    let sum = add(5, 3);

    println!("Sum: {}", sum);
}

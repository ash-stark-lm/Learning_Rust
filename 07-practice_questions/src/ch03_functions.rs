// Chapter 03 — Functions & Ownership Practice
//
// Fill in the bodies of the functions by replacing `todo!()`.

fn main() {
    // Call any practice function from here while working on them.
    // example_basic_functions();
}

// Q1: Implement a simple add function and use it.
pub fn add(a: i32, b: i32) -> i32 {
    // TODO: return the sum of a and b.
    let add = a + b;
    add
}

pub fn example_basic_functions() {
    // TODO:
    // - Call add with a couple of values.
    // - Print the result.

    let result: i32 = add(4, 5);
    println!("{}", result);
}

// Q2: Two greeting versions: one taking String, one taking &str.
pub fn greet_owned(name: String) {
    // TODO: print a greeting; this function takes ownership of name.

    todo!();
}

pub fn greet_borrowed(name: &str) {
    // TODO: print a greeting; this function borrows the string slice.
    todo!();
}

// Q3: Show the difference between passing Copy and non-Copy types by value.
pub fn takes_i32_value(x: i32) {
    // TODO: maybe print x + 1; x is Copy.
    todo!();
}

pub fn takes_string_value(s: String) {
    // TODO: print the string; note that ownership is moved into this function.
    todo!();
}

// Q4: Use references and mutable references as parameters.
pub fn increment_by_mut_ref(x: &mut i32) {
    // TODO: increment the value via mutable reference.
    todo!();
}

pub fn print_via_ref(s: &String) {
    // TODO: print the string without taking ownership.
    todo!();
}

// Q5: Return the maximum of two i32 values.
pub fn max(a: i32, b: i32) -> i32 {
    // TODO: return the larger of a and b.
    todo!();
}

// Q6: Demonstrate cloning a String before passing to a function.
pub fn takes_string_and_prints(s: String) {
    // TODO: print the string; ownership is taken here.
    todo!();
}

pub fn clone_demo() {
    // TODO:
    // - Create a String.
    // - Pass a clone into takes_string_and_prints.
    // - Show that the original is still usable afterwards.
    todo!();
}

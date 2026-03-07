// Chapter 01 — Basic & Workspace Practice
//
// Each block below is a separate coding exercise.
// Replace the `todo!()` calls with your own implementation.

fn main() {
    basic_hello();
    crate_info();
}

// Q1: Write a minimal Rust program that prints "Learning Rust!" to the console.
pub fn basic_hello() {
    println!("Learning Rust!");
}

// Q2: Print the name of the current crate and a short description.
// (Hard‑code the values; focus on using println! correctly.)
pub fn crate_info() {
    println!("practice_ch01_basic");
}

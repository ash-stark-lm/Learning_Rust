/*
When you pass a value to a function in Rust, ownership behavior
depends on the type of the value.

1️⃣ Stack vs Heap Concept:

- `i32` → fixed size, stored entirely on the stack.
  Copying it is cheap because it only copies a small,
  known-size value directly from stack to stack.

- `String` → stores its actual data on the heap.
  The stack only stores (pointer, length, capacity).
  Copying a String would require allocating new heap memory
  and copying the heap data, which is more expensive.
  Therefore, Rust moves ownership instead of copying it by default.
*/

// Takes ownership of a String.
// String does NOT implement `Copy`, so it is moved.
fn takes_ownership(s: String) {
    println!("Inside takes_ownership: {}", s);
}
// `s` is dropped here and its heap memory is freed.

// Takes an i32.
// i32 implements the `Copy` trait, so it is copied.
fn makes_copy(some_integer: i32) {
    println!("Inside makes_copy: {}", some_integer);
}
// `some_integer` goes out of scope, nothing special happens.

fn main() {
    let s = String::from("hello");

    // Ownership of `s` is moved into the function
    // because String does not implement Copy.
    takes_ownership(s);

    // ❌ `s` is no longer valid here.
    // println!("{}", s);

    let x: i32 = 5;

    // `x` is copied because i32 implements Copy
    // and lives entirely on the stack.
    makes_copy(x);

    // ✅ `x` is still valid.
    println!("x is still valid after makes_copy: {}", x);

    let t = String::from("hello");

    // `.clone()` creates a deep copy:
    // - Allocates new heap memory
    // - Copies the data
    // The cloned value is moved into the function,
    // while original `t` remains valid.
    takes_ownership_and_clones(t.clone());

    println!("t is still valid after clone: {}", t);
}

// Takes ownership of the cloned String.
fn takes_ownership_and_clones(s: String) {
    println!("Inside function with clone: {}", s);
}
// The cloned String is dropped here.

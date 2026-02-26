/*
=================================================
            PARAMETER PASSING IN RUST
=================================================

Rust supports:
1) Passing by value
2) Passing by reference (&T)
3) Passing by mutable reference (&mut T)
*/

/* ------------------ Passing by Value ------------------

For Copy types (i32, bool, etc.):
- A bitwise copy is made.
- Original value remains valid.
*/

fn takes_value_copy(mut num: i32) {
    num += 1;
    println!("Inside function: {}", num);
}

/* For heap types like String:
- Passing by value moves ownership.
- Original variable becomes invalid.
*/

fn takes_ownership(s: String) {
    println!("Inside function: {}", s);
}

/* ------------------ Passing by Reference ------------------

& T → Borrowing (read-only)
- No ownership transfer
- Cannot modify
*/

fn calculate_length(s: &String) -> usize {
    s.len()
}

/* ------------------ Passing by Mutable Reference ------------------

&mut T → Borrowing with modification
- No ownership transfer
- Can modify the original value
*/

fn takes_mutable_reference(s: &mut String) {
    s.push_str(", world");
}

/* ------------------ MAIN ------------------ */

fn main() {
    // Passing Copy type
    let x = 5;
    takes_value_copy(x);
    println!("After call: {}", x); // still valid

    println!("------------------");

    // Passing ownership (move)
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("{}", s1); // ❌ invalid (moved)

    println!("------------------");

    // Passing immutable reference
    let s2 = String::from("borrowed");
    let len = calculate_length(&s2);
    println!("Length of '{}' is {}", s2, len); // still valid

    println!("------------------");

    // Passing mutable reference
    let mut s3 = String::from("hello");
    takes_mutable_reference(&mut s3);
    println!("After modification: {}", s3);
}

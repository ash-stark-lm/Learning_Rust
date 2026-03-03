/* To allow a function to use a value without taking ownership, you can pass a reference to the value
 This is known as borrowing. Borrowing allows multiple parts of your code to access a value
without transferring ownership.

We have two kinds of borrowing in functions:

- Immutable borrows --> When you pass an immutable reference to a function, the function can read the value but cannot
modify it. Immutable references are created using the & symbol: &T

- Mutable borrows --> When you pass a mutable reference to a function, the function can modify the value. Mutable
references are created using the &mut symbol: &mut T
*/

fn main() {
    let s = String::from("hello");
    takes_reference(&s);
    println!("{}", s); // s is still valid here
    let mut t = String::from("hello");

    takes_mutable_reference(&mut t);
    println!("{}", t); // t is still valid here
}

//Immutable borrow
fn takes_reference(some_string: &String) {
    println!("{}", some_string);
}
//Mutable borrow-> The takes_mutable_reference function borrows the some_string parameter by taking
//a mutable reference to it

fn takes_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

/*Rust enforces a strict rule to prevent data races: you can only have one mutable
reference to a particular piece of data in a particular scope at any given time. You
also cannot have any immutable references if a mutable reference exists. The com-
piler will stop you if you try to break this rule. This is one of Rust’s most important
safety guarantees. */

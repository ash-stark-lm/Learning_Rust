// Chapter 06 — Functions in Depth (Ownership, Borrowing, Returning) Practice
//
// Implement these functions by replacing `todo!()`.

fn main() {
    // Call any function you want to test here, e.g.:
    // rectangle_demo();
}

// Q1: Compute and print area and return perimeter of a rectangle.
pub fn rectangle_area(width: i32, height: i32) {
    // TODO:
    // - Compute the area and print it.
    let area = width * height;
    println!("The area of the rectangle is {}", area);
}

pub fn rectangle_perimeter(width: i32, height: i32) -> i32 {
    // TODO:
    // - Compute and return the perimeter.
    let perimeter = 2 * (width + height);
    perimeter
}

// Q2: Functions that give and take ownership of a String.
pub fn gives_ownership() -> String {
    // TODO:
    // - Create a String and return it to the caller.
    todo!();
}

pub fn takes_and_returns(s: String) -> String {
    // TODO:
    // - Take ownership of s and then return it.
    todo!();
}

// Q3: Borrowing — immutable and mutable.
pub fn print_immutable_borrow(s: &String) {
    // TODO: print the string without taking ownership.
    todo!();
}

pub fn append_with_mut_borrow(s: &mut String) {
    // TODO: append some text to the string via mutable reference.
    todo!();
}

// Q4: Show ownership transfer through multiple functions.
pub fn ownership_chain_demo() {
    // TODO:
    // - Create a String.
    // - Pass it through gives_ownership and takes_and_returns (or similar).
    // - Finally print it in main chain function to show ownership flow.
    todo!();
}

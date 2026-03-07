// Chapter 02 — Data Types & Structures Practice
//
// Each function corresponds to one or more practice questions.
// Replace the `todo!()` calls with working code.

fn main() {
    // Call the functions you want to test while practicing, e.g.:
    // scalar_demo();
    // tuple_sum_and_diff();
    // array_print_all();
    // slice_first_last();
    // string_vs_str_demo();
}

// Q1–Q2: Declare one variable of each main scalar type and print them.
pub fn scalar_demo() {
    // TODO:
    // - i32 (or another signed integer)
    // - u8 (or another unsigned integer)
    // - f64
    // - bool
    // - char
    // Then print each on its own line.
    let x: i32 = 10;
    let y: u8 = 200;
    let f: f64 = 3.14;
    let b: bool = true;
    let c: char = 'R';

    println!("x: {}", x);
    println!("y: {}", y);
    println!("f: {}", f);
    println!("b: {}", b);
    println!("c: {}", c);
}

// Q3–Q4: Use a tuple to group values and return multiple results.
pub fn tuple_sum_and_diff(a: i32, b: i32) -> (i32, i32) {
    // TODO:
    // - Compute the sum and difference of a and b.
    // - Return them as a tuple (sum, diff).
    let diff = a - b;
    let sum = a + b;
    let result = (sum, diff);
    result
}

// Q5: Declare an array and print all elements using indexing.
pub fn array_print_all() {
    // TODO:
    // - Create an array of 5 i32 values.
    // - Print all elements using indices (not a loop).
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
    println!("{}", arr[3]);
    println!("{}", arr[4]);
}

// Q6: Work with slices and safely print first and last elements.
pub fn slice_first_last(slice: &[i32]) {
    // TODO:
    // - If slice is empty, print a message and return.
    // - Otherwise, print the first and last elements.
    todo!();
}

// Q7: Demonstrate the difference between &str and String.
pub fn string_vs_str_demo() {
    // TODO:
    // - Create an immutable &str.
    // - Create a mutable String and append some text.
    // - Print both values.
    todo!();
}

// Q8: Define and use a classic struct.
pub struct Book {
    // TODO: add fields, e.g. title, pages, available.
}

pub fn book_demo() {
    // TODO:
    // - Construct a Book instance.
    // - Print its fields.
    todo!();
}

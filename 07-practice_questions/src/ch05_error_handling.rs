// Chapter 05 — Error Handling Practice
//
// Implement the functions by replacing `todo!()`.

fn main() {
    // Call any of your practice functions here, e.g.:
    // parse_demo();
}

// Q1: Parse a valid and invalid number using unwrap / expect.
pub fn parse_demo() {
    // TODO:
    // - Parse a valid numeric string with .unwrap().
    // - Parse an invalid numeric string with .expect("...").
    // - Observe (or reason about) the behavior.
    todo!();
}

// Q2: Division that returns Result<f64, String>.
pub fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    // TODO:
    // - If divisor is zero, return Err with a message.
    // - Otherwise, return Ok(result).
    todo!();
}

pub fn divide_demo() {
    // TODO:
    // - Call divide with a non-zero divisor and handle Ok/Err via match.
    // - Call divide with divisor 0.0 and handle the error case.
    todo!();
}

// Q3: Linear search using Option<usize>.
pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    // TODO:
    // - Return Some(index) if target is found, else None.
    todo!();
}

pub fn linear_search_demo() {
    // TODO:
    // - Call linear_search with an array and print whether the value was found.
    todo!();
}

// Q4: Destructure an error from a Result.
pub fn parse_with_error_destructuring(input: &str) {
    // TODO:
    // - Try to parse input into i32.
    // - Use match to print either the value or the error message.
    todo!();
}

// Q5: Use panic! for unrecoverable error on division by zero.
pub fn divide_or_panic(dividend: f64, divisor: f64) -> f64 {
    // TODO:
    // - If divisor is zero, call panic! with a message.
    // - Otherwise, return the result.
    todo!();
}


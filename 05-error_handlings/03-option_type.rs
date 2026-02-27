/*The Option type is used when a value can be either something or nothing. It is an enum with two
variants: Some (containing a value) and None (no value). The Option type is useful for functions that
might not return a value, providing a safe way to handle absence without resorting to null values */

// lets implement linear search using option type
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let target = 3;
    match linear_search(&numbers, target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found in the array", target),
    }

    unwrap_or();
}

fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &element) in arr.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

/*If we want to get the value inside Some or use a default value if it’s
None, Rust provides a convenient method called unwrap_or(). This can make your code more
concise than a full match block. */

fn unwrap_or() {
    let maybe_number: Option<i32> = Some(10);
    let number = maybe_number.unwrap_or(0); // If maybe_number is None, it will return 0
    println!("The number is: {}", number);
    let nothing: Option<i32> = None;
    let default_number = nothing.unwrap_or(5); // If nothing is None, it will return 5
    println!("The default number is: {}", default_number);
}

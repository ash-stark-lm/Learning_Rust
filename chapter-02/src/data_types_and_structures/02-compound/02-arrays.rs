/*Arrays are collections of multiple values that must all have the same type. In Rust, an array is an
owned type, which means the data it contains is stored directly as part of the array itself (usually
contiguously on the stack). A key characteristic of arrays is that they have a fixed length, which
is known at compile time and cannot be changed once declared. */

fn main() {
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Array: {:?}", array);
    let array2: [i32; 3] = [1, 2, 3];
    println!("Array values: {} {} {}", array2[0], array2[1], array2[2]);
}

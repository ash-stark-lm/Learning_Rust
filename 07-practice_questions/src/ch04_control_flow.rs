// Chapter 04 — Control Flow Constructs Practice
//
// Replace `todo!()` with working implementations.

fn main() {
    // Call the exercises you are working on here.
    // e.g. sign_of_number(5);
}

// Q1: Print whether a number is negative, zero, or positive.
pub fn sign_of_number(n: i32) {
    // TODO: use if / else if / else to print the sign of n.
    if n > 0 {
        println!("positive");
    } else if n < 0 {
        println!("negative");
    } else {
        println!("zero");
    }
}

// Q2: Countdown using a while loop and then print "Go!".
pub fn countdown(start: i32) {
    // TODO:
    // - Use a while loop to count down from start to 1.
    // - Then print "Go!".
    let mut count = start;
    while count >= 1 {
        println!("count");
        count -= 1;
    }
    println!("Go!");
}

// Q3: Print all elements in an array using a for loop.
pub fn print_array_elements(arr: &[i32]) {
    // TODO: use a for loop to print each element of arr.

    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}

// Q4: Demonstrate ranges 1..5 and 1..=5.
pub fn range_demo() {
    // TODO:
    // - Use a for loop with 1..5 (exclusive end).

    for i in 1..5 {
        println!("{}", i);
    }
    // - Use a for loop with 1..=5 (inclusive end).
    for i in 1..=5 {
        println!("{}", i);
    }
    todo!();
}

// Q5: Print a 3x3 matrix using nested loops.
pub fn print_matrix(matrix: &[[i32; 3]; 3]) {
    // TODO:
    // - Use nested for loops to print all elements in grid form.
    for i in 0..3 {
        for j in 0..3 {
            println!("{}", matrix[i][j]);
        }
    }
}

// Q6: Use an enum and match to describe an action.
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn traffic_action(light: TrafficLight) {
    // TODO:
    // - Use match on light to print what the driver should do.
    todo!();
}

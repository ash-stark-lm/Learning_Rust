fn main() {
    single_owner_example();
    moving();
}

//each value has a unique owner responsible for the value’s life cycle, including its access and cleanup.

fn single_owner_example() {
    let s1 = String::from("I am above");
    println!("{}", s1); // s1 is valid here
    {
        let s2 = String::from("I am below");
        println!("{}", s2); // s2 is valid here
    } // s2 is dropped here and the memory is freed

    println!("{}", s1); // s1 is valid here
}

// Move
/* Ownership can be transferred from one variable to another. This process is called “moving.” When a value is moved, the original owner no longer owns the value, and it becomes invalid. The new
owner now controls the value’s life cycle. */
fn moving() {
    let s1 = String::from("I am above");
    println!("{}", s1); // s1 is valid here
    let s2 = s1; // s1 is no longer valid here
    println!("{}", s2);
}

// Why do we need to move the ownership??
/*
Moving ownership is especially useful for large data structures you don’t want to copy. For example, if a function takes ownership of a vector with millions of numbers,
 Rust does not copy all the heap data. It only moves the small stack metadata—the pointer, length, and capacity—making the operation very fast.
*/

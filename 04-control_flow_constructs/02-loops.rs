/*The loop keyword creates an infinite loop that will run forever until you explicitly tell it to stop.  */

fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter is now: {}", counter);
        if counter == 5 {
            break; // Exits the loop
        }
        println!("Loop finished.");
    }

    for_loop_example();
    while_loop_example();
}

// for loop
fn for_loop_example() {
    for i in 1..=5 {
        println!("i is: {}", i);
    }
}

// while loop
fn while_loop_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");
}

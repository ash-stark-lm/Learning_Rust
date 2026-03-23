/*
The `move` keyword forces the closure to take ownership of the variables
it uses from the surrounding environment.

This is important when creating a new thread because the new thread may
run after the original scope ends. If the closure only borrowed the data,
it could lead to a dangling reference.

By using `move`, we transfer ownership of the captured variables into
the closure so the thread safely owns its data.
*/

use std::thread;

fn main() {
    // Create a String that will be used inside the new thread
    let message = String::from("Data for the new thread");

    /*
    Spawn a new thread.

    The `move` keyword moves ownership of `message` into the closure.
    This ensures the new thread owns the data it needs instead of
    borrowing it from the main thread.
    */
    let handle = thread::spawn(move || {
        // The closure can use `message` because it now owns it
        println!("Thread received: {}", message);
    });

    /*
    Wait for the spawned thread to finish execution.

    `join()` blocks the main thread until the spawned thread completes.
    `unwrap()` is used here to panic if the thread fails.
    */
    handle.join().unwrap();
}

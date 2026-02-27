/*
Pattern matching with the match statement is one of Rust’s most powerful features.
It allows you to handle complex control flow by matching values against patterns and executing code based
on which pattern is matched. The match statement can match literals, variables, and wildcards,
and can even destructure structs and enums. */

//match literals directly in a match statement.
fn matching_literals() {
    let num: i32 = 2;
    match num {
        1 => println!("The number is {}", num),
        _ => println!("All other scenarios"),
    }
}

// Matching with variables:the match statement matches the tuple pair and binds its elements to x and y.
//The additional if conditions (called guards) allow for more complex matching logic.
fn matching_with_variables() {
    let pair: (i8, i8) = (-3, 3);
    match pair {
        (x, y) if x == y => println!("Both equal"),
        (x, y) if x + y == 0 => println!("Sum is zero"),
        (x, y) => println!("Different numbers: ({}, {})", x, y),
    }
}

/// Destructuring Enums:-Pattern matching is particularly powerful with enums, it allows us to destructure and handle
//each variant differently.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn destructuring_enums() {
    let msg = { Message::Move { x: (20), y: (12) } };
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change color to red:
{}, green: {}, blue: {}",
            r, g, b
        ),
    }
}

fn main() {
    matching_literals();
    matching_with_variables();
    destructuring_enums();
}

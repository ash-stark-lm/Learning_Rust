/*
Enums in Rust can:
1. Store different types of data in variants
2. Have methods defined using `impl`
3. Be used to model commands or actions in a type-safe way
*/

/* =========================================================
Example 1: Enum with Associated Data + Methods
========================================================= */

enum Message {
    // Variant with no associated data
    Quit,

    // Struct-like variant with named fields
    Move { x: i32, y: i32 },

    // Tuple-like variant holding a String
    Write(String),

    // Tuple-like variant holding three integers
    ChangeColor(i32, i32, i32),
}

// Implement methods for the Message enum
impl Message {
    // Method that operates on &self (borrows the enum value)
    fn call(&self) {
        // Match on the variant of self
        match self {
            Message::Quit => println!("Quit message"),

            // Destructure struct-like variant
            Message::Move { x, y } => {
                println!("Move to x: {}, y: {}", x, y)
            }

            // Destructure tuple-like variant
            Message::Write(text) => {
                println!("Write message: {}", text)
            }

            // Destructure tuple-like variant with 3 values
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red: {}, green: {}, blue: {}", r, g, b)
            }
        }
    }
}

/* =========================================================
Example 2: Enum Used to Model Commands
========================================================= */

enum Command {
    Start,
    Stop,
    Move(i32, i32),
}

// Function that processes commands
fn process_command(command: Command) {
    match command {
        Command::Start => println!("Starting..."),
        Command::Stop => println!("Stopping..."),
        Command::Move(x, y) => {
            println!("Moving to coordinates: x = {}, y = {}", x, y)
        }
    }
}

/* =========================================================
Main Function
========================================================= */

fn main() {
    // ----- Using Message enum with method -----

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello Rust"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    // Calling method defined in impl block
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    println!("----------------------------------");

    // ----- Using Command enum with match processing -----

    process_command(Command::Start);
    process_command(Command::Move(5, 15));
    process_command(Command::Stop);
}

/*
 Entry point of the program.

 Shows:
 - Passing arguments to functions
 - Returning values
 - Functions returning the unit type ()
*/
fn main() {
    let width: i32 = 30;
    let height: i32 = 50;

    calculate_area_of_rectangle(width, height);

    let p = perimeter_of_rectangle(width, height);
    println!("The perimeter of the rectangle is {}", p);

    log_message_implicit("hello");
    log_message_explicit("world");

    demo();
}

/*
 Computes and prints the area of a rectangle.

 Parameters:
 - width
 - height

 Does not return a value.
*/
pub fn calculate_area_of_rectangle(width: i32, height: i32) {
    let area: i32 = width * height;
    println!("The area of the rectangle is {}", area);
}

/*
 Computes and returns the perimeter of a rectangle.

 Returns:
 - i32 perimeter

 The final expression is returned automatically.
*/
fn perimeter_of_rectangle(width: i32, height: i32) -> i32 {
    let perimeter: i32 = 2 * (width + height);
    perimeter
}

/*
 Prints a log message.

 Implicitly returns the unit type ().
*/
fn log_message_implicit(message: &str) {
    println!("[LOG] {}", message);
}

/*
 Prints a log message.

 Explicitly specifies the unit return type ().
*/
fn log_message_explicit(message: &str) -> () {
    println!("[LOG] {}", message);
}

/*
 Simple function with no parameters and no return value.

 Demonstrates a basic void-style function in Rust.
*/
fn demo() {
    print!("Hola! Soy Leo Messi");
}

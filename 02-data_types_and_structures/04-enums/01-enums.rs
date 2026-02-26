/*Enums, short for enumerations, are a powerful feature in Rust that allow you to define a type by
enumerating its possible variants. Enums are particularly useful when you need to work with a
value that can be one of several distinct types */

/*Enums can hold different types of data. Each variant of an enum can have associated data of
different types and structures, much like a struct */
#[derive(Debug)]
// The `Debug` trait allows us to print the enum using {:?}.
// It automatically generates an implementation of the Debug trait
// so we can use println!("{:?}", value).
// Without this, trying to print with {:?} would cause a compile-time error.
#[allow(dead_code)] // This attribute tells the Rust compiler to ignore warnings about unused code in this file.
enum Color {
    // A variant that holds a tuple of three 8-bit unsigned integers
    Rgb(u8, u8, u8),

    // A variant that holds a single String
    Named(String),
}

fn main() {
    let red = Color::Rgb(255, 0, 0);
    let custom_color = Color::Named(String::from("Forest Green"));

    println!("An RGB color: {:?}", red);
    println!("A named color: {:?}", custom_color);
}

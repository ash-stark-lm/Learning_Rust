/* This is a growable, heap-allocated data structure. It is mutable and can store a dynamic number
of characters.*/

fn main() {
    // Create a String from a string literal
    //This is an immutable reference to a string slice. It can refer to a part of a String or a string literal.
    let x: &str = "message";
    let s: String = String::from("Hello, Rust!");
    print!("Hello {} ", x);
    println!("{}", s);

   

    // To modify the string, we need to declare it as mutable
    let mut a: String = String::from("Hello");
    a += ", World!";
    println!("{}", a);
}

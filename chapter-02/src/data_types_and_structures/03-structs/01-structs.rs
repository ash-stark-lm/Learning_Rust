/*Structs are a fundamental feature in Rust that allow you to create custom data types. Structs
group together related data, allowing you to create complex data structures with named fields. */
// Rust has three types of structs: classic structs, tuple structs, and unit structs.

fn main() {
    classic_struct();
    tuple_struct();
    unit_struct();
}

//classic structs are the most commonly used type of struct. They allow you to define a data structure with named fields. Each field in a struct can have a different type, and you can access these
//fields using dot notation

struct User {
    username: String,
    age: u8,
    active: bool,
}

fn classic_struct() {
    let user1 = User {
        username: String::from("Alice"),
        age: 30,
        active: true,
    };

    println!("Username: {}", user1.username);
    println!("Age: {}", user1.age);
    println!("Active: {}", user1.active);
}
/*Tuple structs are similar to classic structs but use unnamed fields. They are useful when you want
to group a few values together without needing named fields. */
struct Color(i32, i32, i32);
fn tuple_struct() {
    let black = Color(0, 0, 0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
}

/*Unit structs are the simplest form of structs and do not have any fields. They are useful for creating
types that don’t need to store data but still need to implement certain traits */
//e.g struct ReadOnly , struct ReadWrite;

struct AlwaysEqual;
fn unit_struct() {
    let _subject = AlwaysEqual;
}

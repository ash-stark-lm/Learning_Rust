/*
When you pass a parameter to a function, Rust’s ownership model determines whether the func-
tion takes ownership of the parameter or borrows it. By default, parameters passed by value
transfer ownership to the function
*/

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // This line would cause a compile-time error because s is no longer valid
}
/*
The takes_ownership function takes ownership of `some_string`.
After the function call, `s` is no longer valid in `main`, and
using it results in a compile-time error.

Since ownership was moved, the function becomes responsible
for the value, and it is automatically cleaned up when it
goes out of scope. This prevents memory leaks and other
common memory-related issues.
*/
fn takes_ownership(some_string: String) {
    // this function is now the owner of some_string
    println!("{}", some_string);
}

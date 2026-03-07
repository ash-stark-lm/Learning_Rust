/* A closure is an anonymous function that you can define inline, store in a variable, pass
as an argument, or return from another function.
It is kinda similar to “lambda expressions” or “lambdas” from languages such as Python,
Java, or C++, or “arrow functions” in JavaScript, you’ll find that Rust’s closures serve a very similar
purpose. */

/*
e,g c++ lambda functions

int main() {
    auto add_one = [](int x) { return x + 1; };
    std::cout << "5 + 1 = " << add_one(5) << std::endl;
    auto multiply = [](int a, int b) { return a * b; };
    std::cout << "3 * 4 = " << multiply(3, 4) << std::endl;
}
 */

fn main() {
    // The compiler infers that `x` is an i32 and the return type is i32.
    let add_one = |x: i32| x + 1;
    // This is same as defining a function:
    // fn add_one(x: i32) -> i32 {
    //     x + 1
    // }
    println!("5 + 1 = {}", add_one(5));
    // You can also add explicit type annotations for clarity.
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("3 * 4 = {}", multiply(3, 4));
}

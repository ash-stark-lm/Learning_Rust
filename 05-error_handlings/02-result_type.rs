fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    test();
}

//basically if we do let a= 10: a is pointing to a meory which has a value of 10
//Now we did b=a; => now b will point to that memory and a will be freed owner ship of value 10 is moved to b
fn test() {
    let mut x = 32;
    let y = x;
    println!("X is {}", x);
    x += 2;

    // why beacause i:32 follows copy trait stored in stack completely  so both x and y are independent copies of the value 32.
    //When we assign x to y, Rust creates a copy of the value rather than moving ownership.
    //Therefore, modifying x does not affect y, and both variables can be used independently without any ownership issues.
    println!("{}", x);
    println!("Y is {}", y);
    println!("X is {}", x);
}

/*The panic! macro is used to indicate a program failure and immediately terminate execution. It
is typically used in scenarios where the program cannot continue due to an unrecoverable error.
It should be used sparingly and only when absolutely necessary */

fn main() {
    let result = divide(10.0, 0.0);
    if let Err(e) = result {
        panic!("Error occurred: {}", e);
    }
}

fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

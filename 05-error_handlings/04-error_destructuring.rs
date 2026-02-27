/*Error destructuring is the process of extracting the specific error information contained within
the Err variant of a Result type. When an operation fails, the Err variant holds a value that de-
scribes the error. By using pattern matching with match or if let, you can “destructure” the Err
to bind this inner value to a variable, allowing you to inspect it, log it, or handle it in a specific way. */
fn main() {
    let failed_parse: Result<i32, _> = "abc".parse();
    match failed_parse {
        Ok(value) => println!("Parsed value: {}", value),
        Err(e) => println!("Failed to parse: {}", e),
        // Here e is the destructured error.
    }
}

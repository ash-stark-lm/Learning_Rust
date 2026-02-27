/*
.unwrap(): This method is called on the result of an operation. If the operation was suc-
cessful, .unwrap() gives you the successful value. If the operation failed, it will cause
your program to panic and crash.
.expect("error message"): This works exactly like .unwrap(), but it lets you provide
a custom error message that will be displayed when the program panics. This is more
helpful for debugging. */

fn main() {
    // This string can be successfully parsed into a number.
    let number_str = "42";
    // .parse() attempts the conversion. .unwrap() gets the successful value.
    let number = number_str.parse::<i32>().unwrap();
    println!("Successfully parsed number: {}", number);
    // This string CANNOT be parsed into a number.
    let invalid_str = "hello world";
    /*
    Below code will give error of panicked invalid digit
    thread 'main' (6066596) panicked at 01-fundamentals.rs:17:43:
    called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace */
    //let test = invalid_str.parse::<i32>().unwrap();
    // The line below would cause the program to panic and crash.
    // We use .expect() to provide a clear message upon failure.
    // let invalid_number = invalid_str.parse::<i32>()
    // .expect("Failed to parse the string into a number!");
    // Because the line above is commented out, this program will run without error.
    // If you uncomment it, the program will panic and this line will not be reached.
    println!(
        "This line will not be reached if the expect() call
panics."
    );
}

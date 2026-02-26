//Primarily two types: Tuples and Arrays

fn main() {
    tuple();
    let (sum, product) = calculate_sum_and_product(5, 10);
    println!("Sum: {}, Product: {}", sum, product);
}

/*Tuples are a simple way to group together a fixed number of values with a variety of types into a
single compound type. Once declared, a tuple’s length cannot change. They are great for bundling
a few related pieces of data without the need to create a full struct.
*/
fn tuple() {
    let my_tuple: (i32, f64, char) = (42, 3.14, 'R');
    //Print my_tuple using Debug formatting
    print!("Hello ");
    println!("Tuple: {:?}", my_tuple);

    //{} → requires Display
    //{:?} → requires Debug

    //Destructuring a tuple
    // method 1
    //{} is a placeholder.Rust matches placeholders in order.
    let (x, y, z) = my_tuple;
    //auto [x, y, z] = my_tuple; in c++
    println!("Destructured Tuple: x={}, y={}, z={}", x, y, z);
    //similar to cout << "x=" << x << " y=" << y << " z=" << z; in c++;

    // method 2: Diect accessing tuple elements using dot notation
    let x = my_tuple.0;
    let y = my_tuple.1;
    let z = my_tuple.2;
    println!("Destructured Tuple: x={}, y={}, z={}", x, y, z);
}

// Most common use case of tuple is to return multiple values from a function.

fn calculate_sum_and_product(a: i32, b: i32) -> (i32, i32) {
    let sum = a + b;
    let product = a * b;
    (sum, product) // returning a tuple
}

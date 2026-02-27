fn main() {
    the_for_keyword();
    range_with_the_for_loop();
    nested_loops();
}
// The for keyword creates a loop that iterates over a collection of items, such as an array or a range.
/* a.iter() creates an iterator over the elements of the array a.iter() is the method */

fn the_for_keyword() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
        for element in a.iter() {
            //how to change values
            let a = element + 2;
            print!(" {}", a)

            //can't do this
            //element += 2;
            //print!("{}", element);
        }
    }
}

// using ranges with the for loop
fn range_with_the_for_loop() {
    for number in 1..5 {
        println!("Exclusive range value: {}", number);
    }
    for number in 1..=5 {
        println!("Inclusive range value: {}", number);
    }
}

// Nesting Loops:-We can also nest for loops to iterate over multiple collections or ranges simultaneously. This is
//useful for multidimensional data structures such as matrices or grids.

fn nested_loops() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix.iter() {
        for element in row.iter() {
            print!("{} ", element);
        }
        println!();
    }
}

/*Slices provide a way to reference a contiguous sequence of elements within a collection (such as
an array or a Vec) without needing to copy the data. A slice is a view or a borrowed reference
into a portion of that collection. Crucially, slices do not own the data they point to; the ownership remains with the original collection */

fn main() {
    let array = [10, 20, 30, 40, 50];

    // The type of `slice` is `&[i32]`.
    let slice = &array[1..3];
    println!("Original array: {:?}", array);
    println!("Slice (a view into the array): {:?}", slice); // Output: [20, 30]
}

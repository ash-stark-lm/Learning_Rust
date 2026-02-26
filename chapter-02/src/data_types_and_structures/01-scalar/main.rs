//Scalar Types:- Integers, Floating-Point Numbers, Booleans, and Characters
//Compound Types:- Primary Two: Tuples and Arrays

//Integers
//Unsigned: u8, u16, u32, u64, u128 and whether it is signed or unsigned.
//i8-> -128 to 127

fn main() {
    let signed_int: i8 = -12;
    let unsigned_int: u8 = 122;

    println!("Signed Integer: {}", signed_int);
    println!("Unsigned Integer: {}", unsigned_int);

    floating_point_numbers();
    boolean_game();
    characters();
}

//floating-point numbers -> f32 and f64(more precise and default) kinda float and double in C/C++
fn floating_point_numbers() {
    let float_num: f32 = 3.14;
    println!("Floating-point number: {}", float_num);
}

//Booleans

fn boolean_game() {
    let playing: bool = true;
    println!("Playing: {}", playing);
}

//Characters-> single unicode scalar value
fn characters() {
    let letter: char = 'R';
    let emoji: char = '😀';

    println!("Letter: {}, Emoji: {}", letter, emoji);
}

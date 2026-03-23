fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // Both are valid because s1 and s2 both points to differnt memory in heap

    let mut s3 = s1.clone();
    s3.push_str(", World");
    /* The idiomatic Rust approach is to prefer moving or borrowing whenever possible and to use .clone() only when you truly
    need a separate, owned copy of the data.
     */
    println!("s1 = {}, s3 = {}", s1, s3);
}

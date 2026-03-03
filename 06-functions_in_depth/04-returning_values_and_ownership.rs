fn main() {
    let s1 = gives_ownership();
    println!("{}", s1); // now s1 is the owner of the string
    let s2 = String::from("messi");

    let s3 = takes_and_give_back(s2);

    //println!("{}", s2); // cannot do this because s2 is no longer valid
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

/*You can define methods and associated functions for structs to provide behavior associated with
your data types. Methods are defined within an impl block.*/

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    //methods take &self as the first parameter, which allows them to access the fields of the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated functions are defined within an impl block but do not take &self as a parameter. They are often used as constructors or utility functions related to the struct.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("The area of sq is {} square pixels.", sq.area());
}

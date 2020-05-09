#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 32,
        height: 55,
    };
    let rect2 = Rectangle {
        width: 23,
        height: 30,
    };
    println!("rect = {:#?}", rect);
    println!("Area = {}", rect.area());
    println!("Can hold rect2? {}", rect.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 32,
        height: 55,
    };
    println!("rect = {:#?}", rect);
    println!("Area = {}", area(&rect));
}

// 参照渡しすることでムーブされずに済み複数回呼べる
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

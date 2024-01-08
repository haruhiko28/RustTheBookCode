#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // selfの直前に&を使用していることに注意してください。
    // メソッドは、selfの所有権を奪ったり、 ここでしているように不変でselfを借用したり、可変でselfを借用したりできるのです。
    // 他の引数と全く同じですね。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);
    
    let rect1 = Rectangle{width: 30, height: 50};

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(rect1)
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area (rectangle: Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }
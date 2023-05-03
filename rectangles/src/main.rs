#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) ->bool {
        self.length > other.length && self.width > other.width
    }

    /// 연관 함수
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "the area of the rectangle is {} square pixels.",
        area1(length1, width1)
    );

    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2: Rectangle = Rectangle {
        length: 50,
        width: 30,
    };

    println!("rect 2 is {:?}", rect2);
    println!("rect 2 is {:#?}", rect2);

    println!("The area of the rectangle is {} square pixels.", area3(&rect2));
    println!("The area of the rectangle is {} square pixels.", rect2.area());

    let rect3 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect4 = Rectangle {
        length: 45,
        width: 60,
    };
    let rect5 = Rectangle::square(70);

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("Can rect2 hold rect5? {}", rect2.can_hold(&rect5));
}

fn area1(length: u32, width: u32) -> u32 {
    length * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

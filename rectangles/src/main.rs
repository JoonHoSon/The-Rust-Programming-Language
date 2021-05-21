#[derive(Debug)]
struct Retangle {
    length: u32,
    width: u32
}

impl Retangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Retangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // associated function
    fn square(size: u32) -> Retangle {
        Retangle{length: size, width: size}
    }
}

fn main() {
    // let rect1: (u32, u32) = (50, 30);
    let rect1 = Retangle {length: 50, width: 30};
    let rect2 = Retangle{length: 40, width: 10};
    let rect3 = Retangle{length: 45, width: 60};
    let rect4 = Retangle::square(100); // by associated function

    // println!("The area of the retangle is {} square pixels.", area(&rect1));
    println!("The area of the retangle is {} square pixels.", rect1.area());

    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Rect by associated function: {:#?}", rect4);
}

// fn area(retangle: &Retangle) -> u32 {
//     retangle.length * retangle.width
// }


// fn area(demensions: (u32, u32)) -> u32 {
//     demensions.0 * demensions.1
// }
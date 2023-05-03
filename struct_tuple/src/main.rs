struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let red = Color(255, 0, 0);

    println!("black color Red : {}, Green : {}, Blue : {}", black.0, black.1, black.2);
    println!("origin x : {}, y : {}, z : {}", origin.0, origin.1, origin.2);
    println!("red color Red : {}, Green : {}, Blue : {}", red.0, red.1, red.2);
}

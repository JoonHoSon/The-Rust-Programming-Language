fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f/32

    println!("x : {:.5}", x);
    println!("y : {:.5}", y);

    let kor: char = '각';
    let heart_eyed_cat = '😻';

    println!("Korean : {}", kor);
    println!("Heart eyed cat : {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;

    println!("The value of a is : {}", a);
    println!("The value of b is : {}", b);
    println!("The value of c is : {}", c);

    let five_hundred: i32 = tup.0;
    let six_point_four: f64 = tup.1;
    let one: u8 = tup.2;

    println!("Dot accessor index zero : {}", five_hundred);
    println!("Dot accessor index one : {}", six_point_four);
    println!("Dot accessor index two : {}", one);
}

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("space count: {}", spaces);

    // let mut spaces = "      ";
    // spaces = speces.len();
    // expected &str, found usize error 발생

    // let guess = "42".parse().expect("Not a number!");
    // type annotations needed error 발생
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess number: {}", guess);
}

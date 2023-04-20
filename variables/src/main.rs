fn main() {
    /*let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    */

    /*
    let mut x = 5;

    println!("The value of x is : {}", x);

    x = 6;

    println!("The value of x is : {}", x);
    */

    let x = 5;

    let x = x + 1; // 6

    let x = x * 2; // 12
    
    println!("The value of x is : {}", x);

    //
    // mismatched types error
    // expected `&str`, found `usize`
    //

    /*
    let mut spaces = "      ";
    spaces = spaces.len();
    */
}

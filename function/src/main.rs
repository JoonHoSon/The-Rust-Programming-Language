fn main() {
    another_function(5, 6);

    let _x = 5;
    let y = {
        let x = 3;
        x + 1 // return에 해당할 경우 semi colon(;)은 입력하여서는 안됨
    };

    println!("The value of y is: {}", y);

    let five = five();

    println!("The value of five: {}", five);

    let plus_one = plus_one(5);

    println!("The value of plus_one is: {}", plus_one);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y)
}

fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

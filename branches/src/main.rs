fn main() {
    let condition = true;
    let number:i32 = if condition {
        5
    } else {
        6 // 문자열일 경우 error[E0308]: if and else have incompatible types 발생
    };

    println!("The value of number is: {}", number);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..=3).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}

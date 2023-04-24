fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {
        // (condition)? 5: 5;
        5
    } else {
        6
    };

    println!("The value of number is : {}", number);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is : {}", element);
    }

    for number in 1..4 {
        println!("{}<<<", number);
    }

    for number in (1..4).rev() { // reverse
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}

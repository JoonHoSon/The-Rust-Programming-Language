use std::{io, process::exit};

fn main() {
    println!("Input fibonacci sequence number");

    loop {
        println!("Please input end number(num > 0)");

        let mut _input: String = String::new();

        io::stdin().read_line(&mut _input).expect("Please enter number!!!");

        let input_number: u32 = match _input.trim().parse() {
           Ok(num) => num,
            Err(_) => {
                println!("Please type a number!!!");

                exit(0);
            }
        };

        make_sequence(input_number);

        exit(0);
    }
}

fn make_sequence(count: u32) {
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut sum: u32;

    for num in 1..count + 1 {
        if num <= 2 {
            first = 1;
            second = 1;
            println!("{} : 1", num);
        } else {
            sum = first + second;
            first = second;
            second = sum;

            println!("{} : {}", num, sum);
        }
    }
}

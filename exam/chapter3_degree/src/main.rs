use std::{io, process::exit};

fn main() {
    println!("Celsius <-> Fahrenheit convert");

    loop {
        println!("Please input degree");
        println!("1 : Celsius[℃] -> Fahrenheit[℉]");
        println!("2 : Fahrenheit[℉] -> Celsius[℃]");
        println!("3 : exit");

        let mut _type = String::new();

        io::stdin().read_line(&mut _type).expect("Select 1 ~ 3");

        let selected_type: i32 = match _type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");

                continue;
            }
        };

        let mut input_num = String::new();

        match selected_type {
            3 => {
                println!("Program terminated.");

                exit(0);
            }
            1 | 2 => {
                println!("Please input degree(only numer) : ");

                io::stdin()
                    .read_line(&mut input_num)
                    .expect("Please type a number!!!");

                let input_degree: f64 = match input_num.trim().parse::<f64>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please type a only number!!!");

                        continue;
                    }
                };

                if selected_type == 1 {
                    println!("{}℃ to {}℉", input_degree, cel_to_fah(input_degree));
                } else {
                    println!("{}℉ is {}℃", input_degree, fah_to_cel(input_degree));
                }
            }
            _ => {
                println!("Please type 1 ~ 3");

                continue;
            }
        }
    }
}

fn cel_to_fah(degree: f64) -> f64 {
    (degree * 9.0 / 5.0) + 32.0
}

fn fah_to_cel(degree: f64) -> f64 {
    (degree - 32.0) * 5.0 / 9.0
}

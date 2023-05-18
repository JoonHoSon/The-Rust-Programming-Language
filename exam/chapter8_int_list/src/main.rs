use std::collections::HashMap;
use std::io;

const MESSAGE: &str = "Please input list(space separate)";

fn main() {
    println!("{}", MESSAGE);

    let mut input: String = String::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut vector:Vec<i32> = Vec::new();

    io::stdin().read_line(&mut input).expect(MESSAGE);

    for s in input.split(" ") {
        let to_num = match s.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                panic!("{}", MESSAGE);
            }
        };

        let value = map.entry(to_num).or_insert(0);
        *value += 1;

        vector.push(to_num);
    }

    let mut sum = 0;

    for value in &vector {
        sum += value;
    }

    let average = (sum as f64) / (vector.len() as f64);

    vector.sort();

    println!("total : {}", sum);
    println!("average : {}", average);
    println!("sorted : {:?}", vector);

    if vector.len() / 2 != 0 {
        println!("mid value : {}", &vector[vector.len() / 2]);
    } else {
        println!("mid value : {}", &vector[vector.len() / 2 - 1]);
    }

    let mut max_mode_value: i32 = 0;
    let mut max_mode_key: i32 = 0;

    for (key, value) in &map {
        if value > &max_mode_value {
            max_mode_value = *value;
            max_mode_key = *key;
        }
    }

    println!("max mode is {}", max_mode_key);
}

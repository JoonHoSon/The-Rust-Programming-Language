use std::collections::HashMap;
use std::error::Error;
use std::io;

const MESSAGE: &str = "Please input list(space separate)";

fn main() {
    println!("{}", MESSAGE);

    let mut input: String = String::new();
    let mut map: HashMap<String, i32> = HashMap::new();

    io::stdin().read_line(&mut input).expect(MESSAGE);

    println!("Input : {}", input);

    for s in input.split(" ") {
        let key = String::from(s.trim());
        let value  = map.entry(key.clone()).or_insert(0);

        *value += 1;
    }

    println!("{:?}", map);
}
//! 명령어 종류
//! * add John to engineering
//! * show engineering
//! * show all
//! * quit

use std::collections::HashMap;
use std::io;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
const MESSAGE: &str = "
Please enter command
ex1) add John to engineering
ex2) show all
ex3) show engineering
ex4) exit
";

fn main() {
    display_command();
    let mut department: HashMap<String, String> = HashMap::new();
    let mut current_command: String = String::new();
    let mut sub_command = String::new();

    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect(&format!("{}{}", CLEAR, MESSAGE)[..]);

        let command = input.trim().to_lowercase();
        let split_words: Vec<&str> = command.split(" ").collect();
        let first_word: &str = split_words.get(0).expect(&format!("{}{}", CLEAR, MESSAGE)[..]);

        match first_word {
            "add" => {

            },
            "show" => {

            },
            "exit" => {
                break;
            }
            _ => println!("{}{}", CLEAR, MESSAGE),

        }



        // if command == "add" {
        // } else if command == "show" {
        //     show_command();
        // } else if command == "main" {
        //     display_command();
        // } else if command == "exit" {
        //     break;
        // } else {
        //     println!("{}", "Input [add | show]");
        //
        //     continue;
        // }
        //
        // current_command = command;
    }
}

fn display_command() {
    println!("{}", CLEAR);
    println!("{}", MESSAGE);
}

fn show_department_list(map: &HashMap<String, String>) {
    if map.is_empty() {
        println!("No registered departments.")
    }
}

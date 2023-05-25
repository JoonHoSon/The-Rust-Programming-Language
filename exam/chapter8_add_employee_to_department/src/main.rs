//! 명령어 종류
//! * add John to engineering
//!     * 부서명은 모두 소문자로 저장
//!     * `to`가 포함되지 않을 경우 실패
//!     * `to`로 끝날경우 실패
//!     * `add`와 `to` 사이에 이름이 없을 경우 실패
//! * show
//!     * all - 부서별 직원 목록 출력
//!     * 없는 부서명을 입력할 경우 등록되지 않은 부서명이라는 안내 출력
//!     * show engineering
//!     * show all
//! * quit

use std::collections::HashMap;
use std::io;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
const MESSAGE: &str = "
Please enter command
ex1) add John to engineering
ex2) show all
ex3) show engineering
ex4) exit or ctrl-c
";
const ADD: &str = "add";
const TO: &str = "to";
const OMITTED_TO_MESSAGE: &str =
    "Omitted a [to] from command.\nusage : add [employee_name] to [department_name]";
const OMITTED_NAME: &str =
    "Omitted a [employee_name] from command.\nusage : add [employee_name] to [department_name]";
const OMITTED_DEPARTMENT: &str =
    "Omitted a [department_name] from command.\nusage : add [employee_name] to [department_name].";
const SHOW: &str = "show";
const EXIT: &str = "exit";

fn main() {
    display_command();
    let mut department: HashMap<String, String> = HashMap::new();

    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect(&format!("{}{}", CLEAR, MESSAGE)[..]);

        let command = input.trim().to_lowercase();
        let split_words: Vec<&str> = command.split(" ").collect();
        let first_word: &str = split_words
            .get(0)
            .expect(&format!("{}{}", CLEAR, MESSAGE)[..]);

        // println!("split_words : {:?}", split_words);

        match first_word {
            ADD => {
                add_employee(split_words, &mut department);
            }
            SHOW => {}
            EXIT => {
                break;
            }
            _ => display_command(),
        }
    }
}

fn display_command() {
    println!("{}{}", CLEAR, MESSAGE);
}

fn add_employee(words: Vec<&str>, map: &mut HashMap<String, String>) {
    if !words.contains(&TO) {
        display_command();
        println!("{}", OMITTED_TO_MESSAGE);

        return;
    }

    let to_index = words.iter().position(|r| r == &TO).unwrap();

    if to_index == words.len() - 1 {
        display_command();
        println!("{}", OMITTED_DEPARTMENT);

        return;
    }

    if to_index == 1 {
        display_command();
        println!("{}", OMITTED_NAME);
    }

    let name_slice = &words[1..to_index];
    let department_name_slice = &words[to_index + 1..];
    let mut name = String::new();
    let mut department_name = String::new();

    for (i, n) in name_slice.iter().enumerate() {
        if i > 0 {
            name.push_str(" ");
        }

        name.push_str(n);
    }

    for (i, n) in department_name_slice.iter().enumerate() {
        if i > 0 {
            department_name.push_str(" ");
        }

        department_name.push_str(n);
    }

    map.entry(department_name).or_insert(name);

    show_department_list(&map);
}

fn show_department_list(map: &HashMap<String, String>) {
    if map.is_empty() {
        display_command();
        println!("No registered departments.");

        return;
    }

    let sorted: Vec<_> = map.iter().collect();

    println!("{:?}", sorted);
}

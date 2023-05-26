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
use std::{format, io};

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
const OMITTED_TO_MESSAGE: &str = "
Omitted a [to] from command.\nusage : add [employee_name] to [department_name]
";
const OMITTED_NAME: &str = "
Omitted a [employee_name] from command.\nusage : add [employee_name] to [department_name]
";
const OMITTED_DEPARTMENT: &str = "
Omitted a [department_name] from command.\nusage : add [employee_name] to [department_name].
";
const DENIED_USE_KEYWORD: &str = "is reserved keyword.";
const NOT_EXIST_DEPARTMENT: &str = "does not exist.";
const SHOW: &str = "show";
const EXIT: &str = "exit";
const ALL: &str = "all";

fn main() {
    display_command();
    let mut department: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect(&format!("{}{}", CLEAR, MESSAGE)[..]);

        let source = input.trim();
        let command = input.trim().to_lowercase();
        let split_words: Vec<&str> = command.split(" ").collect();
        let first_word: &str = split_words
            .get(0)
            .expect(&format!("{}{}", CLEAR, MESSAGE)[..]);

        match first_word {
            ADD => {
                add_employee(split_words, &source, &mut department);
            }
            SHOW => {
                show_list(&split_words[1..], &department);
            }
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

fn add_employee(words: Vec<&str>, source: &str, map: &mut HashMap<String, Vec<String>>) {
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

    let name_slice: &[&str] = &words[1..to_index];
    let department_name_slice: &[&str] = &words[to_index + 1..];
    let mut name: String = String::new();
    let mut department_name: String = String::new();
    let source_slice: Vec<&str> = source.split(" ").collect();
    let name_source_slice: &[&str] = &source_slice[1..to_index];
    let department_name_source_slice: &[&str] = &source_slice[to_index + 1..];

    for (i, _) in name_slice.iter().enumerate() {
        if i > 0 {
            name.push_str(" ");
        }

        name.push_str(name_source_slice[i]);
    }

    for (i, _) in department_name_slice.iter().enumerate() {
        if i > 0 {
            department_name.push_str(" ");
        }

        department_name.push_str(department_name_source_slice[i]);
    }

    // all은 부서명으로 사용할 수 없음
    if department_name.to_lowercase() == ALL {
        display_command();
        println!("[{}] {}", department_name, DENIED_USE_KEYWORD);

        return;
    }

    let exist_keys: Vec<&String> = map
        .keys()
        .filter(|k| k.to_lowercase() == department_name.to_lowercase())
        .collect();

    if exist_keys.len() > 0 {
        map.get_mut(&exist_keys[0].to_string()).unwrap().push(name);
    } else {
        map.insert(department_name, vec![name]);
    }

    show_department_list(&map);
}

fn show_list(target: &[&str], map: &HashMap<String, Vec<String>>) {
    if format!("{:?}", target).to_lowercase() == ALL {
        println!("show all");
    }
    // if target.to_lowercase() == ALL {
    //     println!("전체 출력");
    // } else {
    //     let exist_keys: Vec<_> = map
    //         .keys()
    //         .filter(|k| k.to_lowercase() == target.to_lowercase())
    //         .collect();

    //     if exist_keys.len() == 0 {
    //         display_command();
    //         println!("[{}] {}", target, NOT_EXIST_DEPARTMENT);
    //     }
    // }
}

fn show_department_list(map: &HashMap<String, Vec<String>>) {
    if map.is_empty() {
        display_command();
        println!("No registered departments.");

        return;
    }

    for (key, value) in map.iter() {
        println!("--------------------------------------------------------");
        println!("Department [{:?}]", key);
        println!("--------------------------------------------------------");

        for n in value {
            println!("{:?}", n);
        }
    }
}

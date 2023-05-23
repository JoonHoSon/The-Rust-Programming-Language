//! 명령어 종류
//! * add : 직원을 특정 부서에 등록
//! * show : 전체 혹은 특정 부서의 직원 목록 출력(이름 정렬)
//! * 명령어 목록 화면으로 이동
//! * quit : 프로그램 종료

use std::collections::HashMap;
use std::io;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
const MESSAGE: &str = "
Please select command.
ex1) add John to engineering
ex2) show all
ex3) show engineering
ex4) exit
";
const SHOW_MESSAGE: &str = "
department : Show departments.
[department name] : Show employees of [department name].
main : Move to main command scree.
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
            .expect("Input [add | show | main | quit]");

        let command = input.trim().to_lowercase();

        if command == "add" {
        } else if command == "show" {
            show_command();
        } else if command == "main" {
            display_command();
        } else if command == "exit" {
            break;
        } else {
            println!("{}", "Input [add | show]");

            continue;
        }

        current_command = command;
    }
}

fn display_command() {
    println!("{}", CLEAR);
    println!("{}", MESSAGE);
}

fn show_command() {
    println!("{}", CLEAR);
    println!("{}", SHOW_MESSAGE);
}

fn show_department_list(map: &HashMap<String, String>) {
    if map.is_empty() {
        println!("No registered departments.")
    }
}

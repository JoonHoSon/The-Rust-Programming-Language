use std::{collections::HashMap, io};

const MESSAGE: &str = "Please input word(space separate)";

fn main() {
    let consonants: Vec<char> = vec![
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let mut map: HashMap<String, String> = HashMap::new();
    let mut input: String = String::new();

    println!("{}", MESSAGE);

    io::stdin().read_line(&mut input).expect(MESSAGE);

    for s in input.split(" ") {
        append_map(&consonants, &vowels, &mut map, s.trim().to_string());
    }


    for (key, value) in &map {
        println!("{} => {}", key, value);
    }
}

fn append_map(
    consonants: &Vec<char>,
    vowels: &Vec<char>,
    map: &mut HashMap<String, String>,
    word: String,
) {
    let first_character: char = word[..1].chars().collect::<Vec<_>>()[0];
    let result: String;

    if consonants.contains(&first_character) {
        result = format!("{}{}ay", &word[1..], &word[..1]);
    } else if vowels.contains(&first_character) {
        result = format!("{}hay", word);
    } else {
        panic!("{}", MESSAGE);
    }

    map.entry(word).or_insert(result);
}

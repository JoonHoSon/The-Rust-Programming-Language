use std::collections::HashMap;

fn main() {
    let mut score: HashMap<String, i32> = HashMap::new();

    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Red"), 50);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name: String = "Favorite color".to_string();
    let field_value = "Blue".to_string();
    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // 이 시점 이후 field_name 및 field_value는 유효하지 않음
    // field_name 및 field_value는 이동(move)됨
    // value borrowed here after move 오류 발생
    // println!("field_name : {}", field_name);
    // println!("field_value : {}", field_value);

    let blue_team_store: Option<&i32> = score.get("Blue");

    assert_eq!(blue_team_store, Some(&10 as &i32));
    assert_eq!(score.get("Yellow"), None);

    for (key, value) in &score {
        println!("{} : {}", key, value);
    }

    score.entry("Yellow".to_string()).or_insert(80);

    println!("{:?}", score);

    let text: &str = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{:?}", map);
}

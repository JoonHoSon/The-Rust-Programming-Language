use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    println!("before field_name : {}", field_name);
    println!("before field_value : {}", field_value);

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(field_name, field_value);

    println!("after field_name : {}", field_name);
    println!("after field_value : {}", field_value);
}

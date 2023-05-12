fn main() {
    let data: &str = "initial contents";
    let s: String = data.to_string();
    let s: String = "initial contents".to_string();

    let mut s1: String = String::from("foo");
    let s2: &str = "bar";

    s1.push_str(&s2);

    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    s1.push('_');

    let s3: char = '!';

    s1.push(s3);

    println!("s1 is {}", s1);

    let s3: char = '한';

    s1.push(' ');
    s1.push(s3);

    println!("s1 is {}", s1);

    print_language_data("Korean", &"안녕하세요".to_string());
    print_language_data("Russian", &"Здравствуйте".to_string());
    print_language_data("Japanese", &"こんにちは".to_string());
    print_language_data("Thai", &"สวัสดีค่ะ".to_string());
    print_language_data("Emoticon", &"👍👏🎉😻".to_string());
}

fn print_language_data(language_name: &str, target: &String) {
    let length = target.len();
    let character_byte_size = length / target.char_indices().count();
    let first_character = &target[0..character_byte_size];

    println!("------------------------------------------------------------------------");
    println!("{} length : {}", language_name, target.len());
    println!("{} hello character count : {}", language_name, target.char_indices().count());
    println!("Unicode byte per {} character :{}", language_name, character_byte_size);
    println!("{} first character : {}", language_name, first_character);
    println!("Iterate {} character", language_name);

    for c in target.chars() {
        print!("[{}] ", c);;
    }

    println!("\nIterate {} bytes", language_name);

    for c in target.bytes() {
        print!("[{}] ", c);
    }

    println!();
}

fn main() {
    let mut s: String = String::from("hello world");
    let first_word = first_word(&s);
    let second_word = second_word(&s);

    println!("first word is {}.", first_word);
    println!("second word is {}.", second_word);

    //s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable 오류

    //println!("the first word is : {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i + 1)..];
        }
    }

    return &s[..];
}

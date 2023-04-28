fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // missing lifetime specifier 오류 발생
    let s = String::from("hello");

    &s
}

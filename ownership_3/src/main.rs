fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}", s1);
    // println!("s2: {}", s2); // takes_and_gives_back 함수로 인해 소유권은 s3이전됨. borrow of moved value: `s2` 오류 발생
    println!("s3: {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
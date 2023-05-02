fn main() {
    let s = String::from("hello");

    takes_ownership(s); // s의 값이 함수 안으로 이동. 현재 스코프에서는 더이상 s가
    // 유효하지 않음

    let x = 5;

    makes_copy(x); // x의 값이 함수 않으로 이동했으나 정수형의 Copy trait으로 인해 이후
    // 스코프에서도 여전히 x는 유효함
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

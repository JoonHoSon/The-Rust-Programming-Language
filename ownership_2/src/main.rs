fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    make_copy(5);

    println!("after string: {}", s); // borrow of moved value: `s` 오류 발생
    println!("after integer: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
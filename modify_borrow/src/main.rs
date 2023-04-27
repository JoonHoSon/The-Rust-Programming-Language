fn main() {
    // let s = String::from("hello");

    // change1(&s); // cannot borrow `*some_string` as mutable, as it is behind a `&` reference 오류
    // 발생
    let mut s = String::from("hello");

    change2(&mut s);

    println!("{}", s);
}

// fn change1(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

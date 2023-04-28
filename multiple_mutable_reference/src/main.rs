fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2); // cannot borrow `s` as mutable more than once at a time 오류 발생
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 pointer가 s2으로 이동. 이 후 s1은 drop 됨

    println!("{}, world!", s1); // borrow of moved value: `s1` 오류 발생
}

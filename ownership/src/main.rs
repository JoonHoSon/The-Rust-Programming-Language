fn main() {
    /*
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // borrow of moved value: `s1` 오류 발생
    */

    let s1 = "hello";
    let s2 = s1;

    println!("{}, world!", s1);
}

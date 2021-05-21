fn main() {
    /*
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // borrow of moved value: `s1` 오류 발생
    */

    // 동적 할당이 아니므로 
    let s1 = "hello";
    let _s2 = s1;

    println!("{}, world!", s1);
}

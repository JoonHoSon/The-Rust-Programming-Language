fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100]; // panic!
    let does_not_exist = v.get(100); // return None

    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v2[0];

    v2.push(6);

    // 예제에서는 이 부분이 빠져 있음
    // https://doc.rust-lang.org/book/ch08-01-vectors.html
    // Listing 8-6: Attempting to add an element to a vector while holding a reference to an item 참고
    // cannot borrow `v2` as mutable because it is also borrowed as immutable 오류 발생
    print!("The first element is : {first}");
}

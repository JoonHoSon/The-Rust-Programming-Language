fn main() {
    // let tup(i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // 구조체 형식으로 마침표(.) 방식으로 접근 가능. 인덱스(0부터 시작) 번호로 접근
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("Five hunred: {}", five_hundred);
    println!("Six point four: {}", six_point_four);
    println!("One: {}", one);
}

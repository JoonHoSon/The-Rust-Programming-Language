#[derive(Debug, PartialEq, Eq)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call {:#?}", self)
    }
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(&home);
    route(&loopback);

    let m = Message::Write(String::from("hello"));

    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // no implementation for `i8 + Option<i8>` 오류 발생
    let sum = x + y.unwrap();

    println!("sum -> {}", sum);
}

fn route(ip_type: &IpAddrKind) {
    match ip_type {
        IpAddrKind::V4(_, _, _, _) => println!("IP v4!"),
        IpAddrKind::V6(_) =>println!("IP v6!")
    }
}
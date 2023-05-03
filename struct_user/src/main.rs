#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2= build_user(String::from("joonho.son@me.com"), String::from("손준호"));

    dbg!(user2);

    // user2 사용시 다음과 같으 오류 발생
    // move occurs because `user2` has type `User`, which does not implment the `Copy` trait
    let user3: User = User {
        sign_in_count: 2,
        active: false,
        ..user1
    };

    dbg!(user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

[The Rust Programming Language(번역본)](https://rinthel.github.io/rust-lang-book-ko/) 예제 코드 모음

# Cargo cli 명령어 모음

```bash
$ cargo doc --open # 프로젝트 및 의존성이 설정된 라이브러리의 문서를 브라우저로 확인 할 수 있다.
```

# 자료형

## Scala type 정리

### 정수형(Signed / Unsigned)

| Length | Signed(부호있음) | Unsigned(부호 없음) |
| :----: | :--------------: | :-----------------: |
|  8bit  |       `i8`       |        `u8`         |
| 16bit  |      `i16`       |        `u16`        |
| 32bit  |      `i32`       |        `u32`        |
|  8bit  |      `i64`       |        `u64`        |
|  arch  |     `isize`      |       `usize`       |

`arch`는 cpu 환경에 따라 `64bit` 혹은 `32bit`로 처리됨. 문서에는 다음과 같이 명시되어 있다(확인 필요).

> `isize`나 `usize`는 일부 콜렉션 타입의 색인에 사용됩니다.

속도면에서는 `i32`가 가장 빠르다고 되어있음.

### 정수형 표현식

| Number literals   | Example                      |
| ----------------- | ---------------------------- |
| 10진수            | `98_222`(세자리 콤마 표현식) |
| 16진수            | `0xff`                       |
| 8진수             | `0o77 `                      |
| 2진수             | `0b1111_0000`                |
| 바이트(`u8` only) | `b'A'`                       |

### 부동 소수점 타입

`f32`와 `f64`가 제공되며, 기본 타입은 `f64`이다.

> 기본 타입은 `f64`인데, 그 이유는 최신의 CPU 상에서는 `f64`가 `f32`와 대략 비슷한 속도를 내면서도 더 정밀한 표현식이 가능하기 때문입니다.

아래의 예제처럼 타입을 명시하지 않았을 경우 (가능한 경우) `f64` 타입으로 할당.

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0 // f32
}
```

# 소유권(Ownership)

생성되는 메모리 대상(`Stack`, `Heap`)에 따라 다르게 작동한다. `Stack`에 적재된 데이터는 자동으로 `Copy trait`이 동작하여 복사가 이루어짐. 아래의
예제는 [사이트](https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html)에 명시된 예제를 응용하였다.

```rust
fn main() {
    let s1 = String::from("hello"); // Heap에 저장됨. 따라서 아래의 println!은 오류
    let s2 = s1;
}

println!("{}, world!", s1); // borrow of moved value: s1 오류 발생
```

문자열이 컴파일 타임에 크기가 결정되므로 `Stack`에 저장되며, `let s2 = s1;` 구문에서 `Copy trait`이 동작하여 깊은 복사(*deep copy*)가 이루어 진다.

```rust
let s1 = "hello"; // Stack에 저장됨
let s2 = s1; // Copy trait이 동작됨

println!("{}, world!", s1); // 정상 출력
```

> 러스트는 정수형과 같이 스택에 저장할 수 있는 타입에 대해 달수 있는 Copy 트레잇이라고 불리우는 특별한 어노테이션(annotation)을 가지고 있습니다 (트레잇에 관해서는 10장에서 더 자세히 보겠습니다). 만일 어떤 타입이 Copy 트레잇을 갖고 있다면, 대입 과정 후에도 예전 변수를 계속 사용할 수 있습니다.

## `Copy trait`이 동작하는 몇 가지 타입

* `u32`와 같은 모든 정수형 타입들
* `true` / `false` 값을 갖는 `bool` 타입
* `f64`와 같은 모든 부동 소수점 타입
* `Copy`가 가능한 타입들로 이루어진 튜플. `(i32, i32)`는 `Copy trait`이 동작하지만 `(i32, String)`은 동작하지 않는다.

# Enum(열거형)

[표준 라이브러리](https://doc.rust-lang.org/std/net/enum.IpAddr.html)에 이미 정의되어 있음.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127.0.0.1);
let loopback = IpAddr::V6(String::from("::1"));
```

# Collection

## String

`ASCII` 문자를 제외한 문자는 모두 `UTF-8`로 처리되며, 흔히 계산하는 문자열의 길이는 `chars().count()`로 처리하여야 한다.

```rust
fn main() {
    let hello1 = "안녕하세요.";
    let hello2 = "Hello.";

    println!("[안녕하세요.] length is {}.", hello1.len());          // [안녕하세요.] length is 16. 반환
    println!("[안녕하세요.] count is {}.", hello1.chars().count()); // [안녕하세요.] count is 6. 반환
    println!("[Hello.] length is {}.", hello2.len());            // [Hello.] length is 6. 반환
    println!("[Hello.] count is {}.", hello2.chars().count());   // [Hello.] count is 6. 반환
}
```

## HashMap

`HashMap<K, V>`에서 Key혹은 Value에 해당하는 값이 `String`일 경우 소유권이 이전됨. `Copy trait`이 구현된 타입(예: `i32`)은 자동으로 값이 복사된다.

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(field_name, field_value);

    println!("after field_name : {}", field_name); // borrow of moved value: `field_name` 오류 발생
    println!("after field_value : {}", field_value); // borrow of moved value: `field_value` 오류 발생
}
```

# 에러처리

* 예제에서는 `RUST_BACKTRACE=1 cargo run`으로 명시되어 있으나, 전체 메세지를 보려면 `RUST_BACKTRACE=full cargo run`으로 실행하여야 한다.
* `ref`는 값을 비교하여 그 참조자를 제공하고 `&`는 참조자 자체를 비교하여 제공한다.
* `Result<T, E>`의 `unwrap` 숏컷 메서드는 `Result<T, E>`가 `Err`일 경우 `panic!`을 호출한다.
* `Result<T, E>`의 `expect` 숏컷 메서드는 `unwrap`과 유사하나 `panic!`을 호출하는 것은 동일하나 문구를 지정할 수 있다.

```rust
use std::fs::File;

fn main() {
    let f1 = File::open("hello.txt").unwrap(); // Error의 시스템 메세지 출력
    let f2 = File::open("hello.txt").expect("Failed to open hello.txt"); // Error에서 지정된 오류 메세지 출력
}
```

## 에러 전파

Java의 `throw`와 같음.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let result: Result<String, io::Error> = read_username_from_file();

    match result {
        Ok(name) => println!("{}", name),
        Err(e) => println!("error -> {:?}", e)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e) // return문으로 에러를 반환하고 종료
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e) // return 생략 가능
    }
}
```

### 에러 전파 숏컷, ?

위 예제의 `read_username_from_file()` 함수를 `?` 숏컷을 이용하여 개선. `?`는 `Result<T, E>` 타입을 반환하는 함수에서만 사용 가능.

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s: String = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

# Generic, Trait, Lifetime

## Trait

Java의 `interface` 개념. 특정 함수에서 `generic` 타입 파라미터에서 다중 `Trait bound`를 특정하는 방법

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    todo!()
}
```

`where` 구문을 이용하여 다음과 같이 표현할 수도 있다.

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    todo!()
}
```

## Lifetime

라이프타임 명시는 다음과 같다.

```rust
&i32         // 일반적인 참조자
&'a i32      // 라이프타임 참조자
&'a mut i32  // 수정 가능한 라이프타임 참조자
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

위의 예제에서는 인자로 전달되는 `x`와 `y` 그리고 반환 타입은 모두 `'a`라는 동일한 라이프타임을 갖는다는 의미이다.

컴파일러가 사용하는 라이프타임 규칙들
> 1. 참조자인 각각의 파라미터는 고유한 라이프타임 파라미터를 갖습니다.
     > 바꿔 말하면, 하나의 파라미터를 갖는 함수는 하나의 라이프타임 파라미터를 갖고: `fn foo<'a>(x: &'a i32)`, 두 개의 파라미터를 갖는 함수는
     > 두 개의 라이프타임 파라미터를 따로 갖고: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 이와 같은 식입니다.
> 2. 만일 정확히 딱 하나의 라이프타임 파라미터만 있다면,
     > 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입됩니다: `fn foo<'a>(x: &'a i32) -> &'a i32`.
> 3. 만일 여러 개의 입력 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 `&self` 혹은 `&mut self`라고 한다면,
     > self의 라이프타임이 모든 출력 라이프타임 파라미터에 대입됩니다. 이는 메소드의 작성을 더욱 멋지게 만들어줍니다.

`ImportantExcerpt.level` 함수의 구현부를 위의 3번 항목을 배제하고 규칙을 적용하면 다음과 같다.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&'a self) -> i32 {
        3
    }
}
```

# Testing

* 단일 스레드(병렬처리X)로 테스트 실행

```bash
$ cargo test -- --test-threads=1
```

* 출력 결과(`println!()` 등)를 포함하여 테스트 실행

```bash
$ cargo test -- --nocapture
```

* 특정 테스트 함수만 실행

```bash
$ cargo test 함수명
```

* 특정 단어(글자)가 포함된 테스트 실행

```bash
$ cargo test 단어(글자) # cargo test add 실행시 'add' 단어가 포함된 모든 테스트 실행
```

* 특정 테스트 함수 무시(ignore)

```rust

#[test]
#[ignore]
fn ignored_test() {
    println!("ignore");
}
```

* 무시(ignore)된 테스트만 모아서 별도로 실행

```bash
$ cargo test --ignored
```

# I/O 프로젝트: 커맨드 라인 프로그램 만들기

`run` 함수에서 반환 타입을 예제와 같이 `Result<(), Box<Error>>`로 할 경우 테스트 환경(`rustc 1.51.0`)에서는 다음과 같은 경고가 출력된다.

```bash
   |
25 | fn run(config: Config) -> Result<(), Box<Error>> {
   |                                          ^^^^^ help: use `dyn`: `dyn Error`
   |
   = note: `#[warn(bare_trait_objects)]` on by default
```

해당 코드를 다음과 같이 변경하면 경고는 사라진다.

```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ...snip...
}
```

`dyn` 키워드는 `Trait`의 `dynamic dispatch`를 적용한다는 의미라는데, 확인이 필요하다([SEORENN님 블로그](https://seorenn.tistory.com/161)).

# Iterator

반복자는 **게으르다** 라는 말은 다음과 같다.

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    v1_iter.map(|x| x + 1);
}
```

위의 소스를 빌드할 경우 다음과 같은 경고가 출력된다.

```bash
 --> src/main.rs:5:5
  |
5 | v1_iter.map(|x| x + 1);
  | ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consume
```

소스상으로는 반복자 어댑터 함수인 `map`을 호출할 경우 인자로 전달된 `|x| x + 1` 클로저가 실행될 것으로 예상하지만, 실제로는 호출되지 않는다.<br>
경고 메세지의 의미는 반복자는 실제 **소비**가 되는 시점에 호출된다는 의미이다.<br>
따라서 `map` 함수는 아래 예제처럼 `collect` 함수가 호출되는 시점에 실행된다.

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let result: Vec<_> = v1_iter.map(|x| x + 1).collect();

    println!("result is {}", result);
}
```
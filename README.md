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

생성되는 메모리 대상(`Stack`, `Heap`)에 따라 다르게 작동한다. `Stack`에 적재된 데이터는 자동으로 `Copy trait`이 동작하여 복사가 이루어짐.
아래의 예제는 [사이트](https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html)에 명시된 예제를 응용하였다.

```rust
let s1 = String::from("hello"); // Heap에 저장됨. 따라서 아래의 println!은 오류
let s2 = s1;

println!("{}, world!", s1); // borrow of moved value: `s1` 오류 발생
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
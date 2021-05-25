/// Copy trait bound 제한. 인자로 전달되는 **&[T]** 형태에서 가장 큰 값을 확인하여
/// Heap memory에 할당(반환 타입이 `T`)하여 반환.
fn largest_v1<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// Copy trait bound 제한 없음. Heap 할당 없음(반환 타입이 `&T`).
fn largest_v2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &*largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_v1(&numbers);

    println!("The largest_v1 number is {}", result);

    let result = largest_v2(&numbers);

    println!("The largest_v2 number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_v1(&chars);

    println!("The largest char is {}", result);

    let result = largest_v2(&chars);

    println!("The largest_v2 char is {}", result);
}

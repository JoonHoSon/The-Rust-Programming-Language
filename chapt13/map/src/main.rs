fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

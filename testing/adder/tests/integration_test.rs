extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    assert_eq!(4, adder::add_two(2), "결과값이 같지 않음");
}
use std::borrow::Borrow;

#[allow(dead_code)]
fn xor_operation1(n:i32, start:i32) -> i32 {
    let mut val = 0;
    for i in 0..n {
        val ^= start+i*2;
    }
    val
}

fn sum_xor(x:i32) -> Option<i32> {
    match x.clone() % 4 {
        0 => Some(x),
        1 => Some(1),
        2 => Some(x + 1),
        3 => Some(0),
        _ => None,
    }
}

fn xor_operation(n:i32, start:i32) -> i32 {
    let s = start.borrow() >> 1;
    let e = n & start & 1;
    let res = sum_xor(s - 1).unwrap() ^ sum_xor(s + n -1).unwrap();
    res << 1 | e
}

fn main() {
    assert_eq!(xor_operation(1, 7), 7);
    assert_eq!(xor_operation(10, 5), 2);
}
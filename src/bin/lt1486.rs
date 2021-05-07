#[allow(dead_code)]
fn xor_operation1(n:i32, start:i32) -> i32 {
    let mut val = 0;
    for i in 0..n {
        val ^= start+i*2;
    }
    val
}

fn sum_xor(x:i32) -> i32 {
    match &x % 4 {
        0 => x,
        1 => 1,
        2 => x + 1,
        _ => 0,
    }
}

fn xor_operation(n:i32, start:i32) -> i32 {
    let s = &start >> 1;
    let e = &n & start & 1;
    let res = sum_xor(s - 1) ^ sum_xor(s + n -1);
    res << 1 | e
}

fn main() {
    assert_eq!(xor_operation(1, 7), 7);
    assert_eq!(xor_operation(10, 5), 2);
}
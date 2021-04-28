use std::collections::HashMap;

#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        map.insert(target - n, i);
    }
    for (i, n) in nums.iter().enumerate() {
        if let Some(&j) = map.get(n) {
            if i != j {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let v = vec![2, 7, 11, 15];
    let target = 9;

    assert_eq!((0, 1), two_sum(v, target).unwrap());

    let v = vec![7, 11, 2, 15];
    let target = 9;
    assert_eq!((0, 2), two_sum(v, target).unwrap());

    let v = vec![3, 4, 2];
    let target = 6;
    assert_eq!((1, 2), two_sum(v, target).unwrap());
}

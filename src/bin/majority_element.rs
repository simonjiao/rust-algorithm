//https://leetcode-cn.com/problems/majority-element/
use std::collections::HashMap;
#[allow(dead_code)]
fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max = 0;
    let mut max_k = 0;
    for n in nums {
        let entry = map.entry(n).or_insert(0usize);
        *entry += 1;
        if *entry > max {
            max_k = n;
            max = *entry;
        }
    }
    max_k
}

#[allow(dead_code)]
fn majority_element_ii(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums[nums.len() / 2]
}

fn majority_element_iii(nums: Vec<i32>) -> i32 {
    let mut idx = 1;
    assert_ne!(nums.len(), 0);

    let mut candidate = nums[0];
    let mut count = 1;
    while idx < nums.len() {
        if candidate == nums[idx] {
            count += 1;
        } else {
            count -= 1;
            if count == 0 {
                count = 1;
                candidate = nums[idx];
            }
        }
        idx += 1;
    }
    candidate
}

fn main() {
    let nums = vec![3, 2, 3];
    assert_eq!(3, majority_element_iii(nums));
}

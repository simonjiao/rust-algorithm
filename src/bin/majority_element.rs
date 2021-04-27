use std::collections::HashMap;

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max =0;
    let mut max_k=0;
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

fn majority_element_ii(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums[nums.len()/2]
}

fn main() {
    let nums = vec![3,2,3];
    assert_eq!(3, majority_element(nums));
}
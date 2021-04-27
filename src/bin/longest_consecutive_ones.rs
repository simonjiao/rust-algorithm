//https://leetcode-cn.com/problems/max-consecutive-ones-iii/

fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = 0;

    while r < nums.len() {
        k -= 1 - nums[r];
        if k < 0 {
            k += 1 - nums[l];
            l += 1;
        }
        r += 1;
    }
    (r - l) as i32
}

fn longest_ones_ii(nums: Vec<i32>, k: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut zeros: i32 = 0;

    while right < nums.len() {
        if nums[right] == 0 {
            zeros += 1;
        }
        // too many zeros, skip all 1s
        if zeros > k {
            if nums[left] == 0 {
                // treat it as 1
                zeros -= 1;
            }
            left += 1;
        }
        right += 1;
    }
    (right - left) as i32
}

fn main() {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;

    assert_eq!(6, longest_ones_ii(nums, k));

    let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;

    assert_eq!(10, longest_ones(nums, k));
}

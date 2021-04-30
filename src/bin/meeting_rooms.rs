//https://leetcode-cn.com/problems/meeting-rooms/

fn overlap(i1: &Vec<i32>, i2: &Vec<i32>) -> bool {
    if (i1[0] >= i2[0] && i1[0] < i2[1]) || (i1[1] > i2[0] && i1[1] <= i2[1]) {
        return true;
    }
    false
}

fn can_attend_meeting(intervals: Vec<Vec<i32>>) -> bool {
    for (idx, m) in intervals.iter().enumerate() {
        let mut idx1 = 0;
        while idx1 < intervals.len() {
            if idx1 != idx {
                if overlap(m, &intervals[idx1]) {
                    return false;
                }
            }
            idx1 += 1;
        }
    }
    true
}

fn main() {
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    assert_eq!(can_attend_meeting(intervals), false);
}

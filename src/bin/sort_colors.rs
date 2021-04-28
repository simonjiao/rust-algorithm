use rust_algorithm::Solution;

//https://leetcode-cn.com/problems/sort-colors/
fn sort_colors(colors: &mut Vec<i32>) {
    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut i = 0;
    while i < colors.len() {
        let c = &mut colors[i];
        match *c {
            0 => c0 += 1,
            1 => c1 += 1,
            2 => c2 += 1,
            _ => panic!("terrible error"),
        }
        i += 1;
    }
    i = 0;
    while i < colors.len() {
        let c = &mut colors[i];
        i += 1;
        if c0 > 0 {
            if *c != 0 {
                *c = 0;
            }
            c0 -= 1;
            continue;
        }
        if c1 > 0 {
            if *c != 1 {
                *c = 1;
            }
            c1 -= 1;
            continue;
        }
        if c2 > 0 {
            if *c != 2 {
                *c = 2;
            }
            c2 -= 1;
            continue;
        }
        panic!("terrible error")
    }
}

fn main() {
    let mut colors = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut colors);
    assert_eq!(vec![0, 0, 1, 1, 2, 2], colors);
}

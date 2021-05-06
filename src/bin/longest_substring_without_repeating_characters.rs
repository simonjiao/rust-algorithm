use std::collections::HashSet;

#[allow(dead_code)]
fn length_of_longest_substring1(s: String)-> i32 {
    let mut set = HashSet::new();
    let mut len = 0;
    let mut r = 0;
    let mut i = 0;
    let size = s.len();
    let c_char = s.as_bytes();

    while i < size {
        if i != 0 {
            set.remove(&c_char[i-1]);
        }
        while r < size && !set.contains(&c_char[r]) {
            set.insert(c_char[r]);
            r += 1;
        }
        len = len.max(r - i);
        i += 1;
    }

    len as i32
}

fn length_of_longest_substring(s: String)-> i32 {
    let mut set = HashSet::new();
    let mut left = 0;
    let mut len = 0;
    let c_char = s.as_bytes();

    for (i, c) in c_char.iter().enumerate() {
        while left < i && set.contains(c) {
            set.remove(&c_char[left]);
            left+=1;
        }
        len = len.max(i+1-left);
        set.insert(&c_char[i]);
    }

    len as i32
}

fn main() {
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("b")), 1);
    assert_eq!(length_of_longest_substring(String::from("")), 0);
}
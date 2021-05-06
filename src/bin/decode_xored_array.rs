#[allow(dead_code)]
fn decode1(encoded: Vec<i32>, first:i32) -> Vec<i32> {
    let mut original = Vec::new();
    original.push(first);

    for n in encoded {
        let &last = original.last().unwrap();
        original.push(last ^ n);
    }

    original
}

fn decode(mut encoded: Vec<i32>, first:i32) -> Vec<i32> {
    let mut last = first;
    encoded.insert(0, 0);
    for n in encoded.iter_mut() {
        *n ^= last;
        last = *n;
    }
    encoded
}

fn main() {
    let encoded = vec![1,2,3];
    let first = 1;
    assert_eq!(decode(encoded, first), vec![1,0,2,1]);


    let encoded = vec![6,2,7,3];
    let first = 4;
    assert_eq!(decode(encoded, first), vec![4,2,0,7,4]);
}
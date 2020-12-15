use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");
    // let input = "0,3,6";

    let mut previous = HashMap::new();

    for (i, n) in input.split(',').map(parse_u).enumerate() {
        previous.insert(n, i);
    }
    let mut next = 0;
    let mut i = previous.len();
    let end = 30000000;
    while i < end - 1 {
        if let Some(prev) = previous.insert(next, i) {
            next = i - prev;
        } else {
            next = 0;
        }
        i += 1;
    }
    pv!(next);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");
    // let input = "0,3,6";

    let mut previous = HashMap::new();

    for (i, n) in input.split(',').map(parse).enumerate() {
        previous.insert(n, i);
    }
    let mut next = 0;
    let mut i = previous.len();
    while i < 2020 - 1 {
        if let Some(prev) = previous.insert(next, i) {
            next = i - prev;
        } else {
            next = 0;
        }
        i += 1;
    }
    pv!(next);
}

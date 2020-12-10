use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input.lines().map(parse).to_vec();
    parsed.sort_unstable();
    parsed.insert(0, 0);
    parsed.push(parsed.last().unwrap() + 3);

    let n = parsed.len();

    let mut ways_to_reach = vec![0u64; n];
    ways_to_reach[0] = 1;

    for index in 1..n {
        let current = parsed[index];
        let mut ways = 0;
        let mut i = index;
        while i > 0 && parsed[i - 1] + 3 >= current {
            i -= 1;
            ways += ways_to_reach[i];
        }
        ways_to_reach[index] = ways;
    }

    pv!(ways_to_reach.last().unwrap());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut parsed = input.lines().map(parse).to_vec();
    parsed.sort_unstable();
    parsed.insert(0, 0);
    parsed.push(parsed.last().unwrap() + 3);
    let mut ones = 0;
    let mut threes = 0;
    for (a, b) in parsed.iter().zip(parsed.iter().skip(1)) {
        let diff = b - a;
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }
    pv!(ones);
    pv!(threes);
    println!("ones * threes: {:?}", ones * threes);
}

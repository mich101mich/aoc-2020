use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{}-{} {}: {}", usize, usize, char, String))
        .filter(|(left, right, c, s)| {
            (s.chars().nth(left - 1).unwrap() == *c) != (s.chars().nth(right - 1).unwrap() == *c)
        })
        .count();
    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| scanf!(l, "{}-{} {}: {}", usize, usize, char, String))
        .filter(|(min, max, c, s)| {
            let cnt = s.chars().filter(|x| x == c).count();
            *min <= cnt && *max >= cnt
        })
        .count();
    pv!(parsed);
}

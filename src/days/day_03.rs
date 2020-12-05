use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");
    let parsed = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').to_vec())
        .to_vec();
    let w = parsed[0].len();

    let mut total = 1;
    for slope in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let mut pos = (0, 0);
        let mut cnt = 0;
        while pos.1 < parsed.len() {
            if parsed[pos.1][pos.0] {
                cnt += 1;
            }
            pos.1 += slope.1;
            pos.0 = (pos.0 + slope.0) % w;
        }
        println!("cnt: {:?}", cnt);
        total *= cnt;
    }
    println!("total: {:?}", total);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");
    let mut pos = (0, 0);

    let parsed = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').to_vec())
        .to_vec();

    let w = parsed[0].len();

    let mut cnt = 0;
    while pos.1 != parsed.len() {
        if parsed[pos.1][pos.0] {
            cnt += 1;
        }
        pos.1 += 1;
        pos.0 = (pos.0 + 3) % w;
    }
    println!("cnt: {:?}", cnt);
}
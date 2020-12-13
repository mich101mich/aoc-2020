use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");
    // let input = "0\n7,13,x,x,59,x,31,19";

    let parsed = input.lines().to_vec();
    let rest = parsed[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| s.parse::<usize>().ok().map(|n| (i, n)))
        .to_vec();

    let (mut offset, mut step) = rest[0];

    for (o, s) in rest.iter().skip(1) {
        println!("{} {}", offset, step);
        let min = (1..)
            .map(|i| i * step + offset)
            .find(|t| (t + o) % s == 0)
            .unwrap();
        offset = min;
        step *= *s;
    }
    pv!(offset);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");
    // let input = ;

    let parsed = input.lines().to_vec();
    let start = parsed[0].parse::<usize>().unwrap();
    let rest = parsed[1]
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .to_vec();

    let mut min = start * 2;
    let mut res = 0;
    for &id in rest.iter() {
        let mut first = (start / id) * id;
        if first < start {
            first += id;
        }
        if first < min {
            min = first;
            res = (first - start) * id;
        }
    }
    pv!(res);
}

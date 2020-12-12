use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let parsed = input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse::<isize>().unwrap()))
        .to_vec();

    let mut pos = (10, -1);
    let mut ship = (0, 0);
    let mut dir = Dir::Right;

    for (c, n) in parsed.iter() {
        let delta = match c {
            'R' => {
                for i in 0..(n / 90) {
                    pos = (-pos.1, pos.0);
                }
                continue;
            }
            'L' => {
                for i in 0..(n / 90) {
                    pos = (pos.1, -pos.0);
                }
                continue;
            }
            'F' => {
                ship.0 += pos.0 * n;
                ship.1 += pos.1 * n;
                continue;
            }
            'N' => Dir::Up.as_delta(),
            'S' => Dir::Down.as_delta(),
            'W' => Dir::Left.as_delta(),
            'E' => Dir::Right.as_delta(),
            c => panic!("{}", c),
        };
        pos.0 += delta.0 * n;
        pos.1 += delta.1 * n;
    }
    pv!(ship);
    pv!(manhattan_i((0, 0), ship));
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let parsed = input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse::<isize>().unwrap()))
        .to_vec();

    let mut pos = (0, 0);
    let mut dir = Dir::Right;

    for (c, n) in parsed.iter() {
        let delta = match c {
            'R' => {
                for i in 0..(n / 90) {
                    dir = dir.clockwise();
                }
                continue;
            }
            'L' => {
                for i in 0..(n / 90) {
                    dir = dir.counter_clockwise();
                }
                continue;
            }
            'F' => dir.as_delta(),
            'N' => Dir::Up.as_delta(),
            'S' => Dir::Down.as_delta(),
            'W' => Dir::Left.as_delta(),
            'E' => Dir::Right.as_delta(),
            c => panic!("{}", c),
        };
        pos.0 += delta.0 * n;
        pos.1 += delta.1 * n;
    }
    pv!(pos);
    pv!(manhattan_i((0, 0), pos));
}

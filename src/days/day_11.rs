use crate::utils::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}
impl Seat {
    fn occupied(&self) -> bool {
        matches!(self, Seat::Occupied)
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    // let input = ;
    let mut parsed = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == 'L' { Seat::Empty } else { Seat::Floor })
                .to_vec()
        })
        .to_vec();

    let w = parsed[0].len();
    let h = parsed.len();
    let deltas = [
        (0isize, -1isize),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let neighborhood = MooreNeighborhood::new(parsed[0].len(), parsed.len());

    loop {
        let mut seats = parsed.clone();
        let mut change = false;
        for (y, row) in parsed.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                if seats[y][x] == Seat::Floor {
                    continue;
                }

                let mut count = 0;
                let max_count = if seat.occupied() { 5 } else { 1 };
                'neighbor: for (dx, dy) in deltas.iter().copied() {
                    let mut sx = x as isize + dx;
                    let mut sy = y as isize + dy;
                    loop {
                        if sx < 0 || sx >= w as isize || sy < 0 || sy >= h as isize {
                            continue 'neighbor;
                        }
                        let s = parsed[sy as usize][sx as usize];
                        match s {
                            Seat::Floor => {}
                            Seat::Empty => continue 'neighbor,
                            Seat::Occupied => {
                                count += 1;
                                if count >= max_count {
                                    break 'neighbor;
                                }
                                continue 'neighbor;
                            }
                        }
                        sx += dx;
                        sy += dy;
                    }
                }
                if count >= max_count && seat.occupied() {
                    seats[y][x] = Seat::Empty;
                    change = true;
                } else if count == 0 && !seat.occupied() {
                    seats[y][x] = Seat::Occupied;
                    change = true;
                }
            }
        }
        parsed = seats;
        if !change {
            break;
        }
    }
    let count = parsed
        .iter()
        .flat_map(|l| l.iter())
        .filter(|s| **s == Seat::Occupied)
        .count();
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    // let input = ;
    let mut parsed = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == 'L' { Seat::Empty } else { Seat::Floor })
                .to_vec()
        })
        .to_vec();

    let neighborhood = MooreNeighborhood::new(parsed[0].len(), parsed.len());

    loop {
        let mut seats = parsed.clone();
        let mut change = false;
        for (y, row) in parsed.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                if seats[y][x] == Seat::Floor {
                    continue;
                }
                let count = neighborhood
                    .get_all_neighbors((x, y))
                    .filter(|&(sx, sy)| parsed[sy][sx] == Seat::Occupied)
                    .count();
                match *seat {
                    Seat::Empty => {
                        if count == 0 {
                            seats[y][x] = Seat::Occupied;
                            change = true;
                        }
                    }
                    Seat::Occupied => {
                        if count >= 4 {
                            seats[y][x] = Seat::Empty;
                            change = true;
                        }
                    }
                    _ => panic!("Floor"),
                }
            }
        }
        parsed = seats;
        if !change {
            break;
        }
    }
    let count = parsed
        .iter()
        .flat_map(|l| l.iter())
        .filter(|s| **s == Seat::Occupied)
        .count();
    pv!(count);
}

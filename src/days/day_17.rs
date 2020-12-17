use crate::utils::*;

#[allow(unused)]
pub fn run() {
    type Point = (isize, isize, isize, isize);
    fn neighbors((x, y, z, w): Point) -> impl Iterator<Item = Point> {
        (-1..=1).flat_map(move |dw| {
            (-1..=1).flat_map(move |dz| {
                (-1..=1).flat_map(move |dy| {
                    (-1..=1)
                        .map(move |dx| (x + dx, y + dy, z + dz, w + dw))
                        .filter(move |p| p.0 != x || p.1 != y || p.2 != z || p.3 != w)
                })
            })
        })
    }
    fn count_iter_is(input: impl Iterator<Item = ()>, target: usize) -> bool {
        let mut count = 0;
        for _ in input {
            count += 1;
            if count > target {
                return false;
            }
        }
        count == target
    }
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");
    //     let input = ".#.
    // ..#
    // ###";
    let mut grid: HashSet<Point> = HashSet::with_capacity(1000);
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                grid.insert((x as isize, y as isize, 0, 0));
            }
        }
    }
    let cycles = 6;
    for i in 0..cycles {
        let mut next = grid.clone();
        let mut seen = grid.clone();
        for cell in grid.iter() {
            let mut count = 0;
            for pos in neighbors(*cell) {
                if seen.insert(pos) {
                    // pos is empty
                    let iter = neighbors(pos).filter(|n| grid.contains(n)).map(|_| ());
                    if count_iter_is(iter, 3) {
                        next.insert(pos);
                    }
                } else if grid.contains(&pos) {
                    count += 1;
                }
            }
            if count < 2 || count > 3 {
                next.remove(cell);
            }
        }
        pv!(next.len());
        grid = next;
    }
}

#[allow(unused)]
pub fn part_one() {
    type Point = (isize, isize, isize);
    fn neighbors((x, y, z): Point) -> impl Iterator<Item = Point> {
        (-1..=1).flat_map(move |dz| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1)
                    .map(move |dx| (x + dx, y + dy, z + dz))
                    .filter(move |p| p.0 != x || p.1 != y || p.2 != z)
            })
        })
    }
    fn count_iter_is(input: impl Iterator<Item = ()>, target: usize) -> bool {
        let mut count = 0;
        for _ in input {
            count += 1;
            if count > target {
                return false;
            }
        }
        count == target
    }
    #[allow(unused)]
    fn print_grid(grid: &HashSet<Point>) {
        let min_z = grid.iter().map(|p| p.2).min().unwrap();
        let max_z = grid.iter().map(|p| p.2).max().unwrap();
        let min_y = grid.iter().map(|p| p.1).min().unwrap();
        let max_y = grid.iter().map(|p| p.1).max().unwrap();
        let min_x = grid.iter().map(|p| p.0).min().unwrap();
        let max_x = grid.iter().map(|p| p.0).max().unwrap();
        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    print!("{}", if grid.contains(&(x, y, z)) { '#' } else { '.' });
                }
                println!();
            }
            println!();
        }
    }
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");
    //     let input = ".#.
    // ..#
    // ###";

    let mut grid: HashSet<Point> = HashSet::with_capacity(1000);

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                grid.insert((x as isize, y as isize, 0isize));
            }
        }
    }

    let cycles = 6;
    for i in 0..cycles {
        let mut next = grid.clone();
        let mut seen = grid.clone();
        for cell in grid.iter() {
            let mut count = 0;
            for pos in neighbors(*cell) {
                if seen.insert(pos) {
                    // pos is empty
                    let iter = neighbors(pos).filter(|n| grid.contains(n)).map(|_| ());
                    if count_iter_is(iter, 3) {
                        next.insert(pos);
                    }
                } else if grid.contains(&pos) {
                    count += 1;
                }
            }
            if count < 2 || count > 3 {
                next.remove(cell);
            }
        }
        // print_grid(&next);
        pv!(next.len());
        grid = next;
    }
}

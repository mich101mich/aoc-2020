use crate::utils::*;

type Point = (isize, isize);

fn neigh_iter(pos: Point) -> impl Iterator<Item = Point> {
    [(1, 0), (-1, 0), (1, 1), (-1, -1), (0, 1), (0, -1)]
        .iter()
        .map(move |delta| (pos.0 + delta.0, pos.1 + delta.1))
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut tiles = HashSet::new();

    for line in input.lines() {
        let mut pos = (0isize, 0isize);
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            let dir = match c {
                'e' => (1, 0),
                'w' => (-1, 0),
                's' => match iter.next() {
                    Some('e') => (0, -1),
                    Some('w') => (-1, -1),
                    x => panic!("Invalid after s: {:?}", x),
                },
                'n' => match iter.next() {
                    Some('e') => (1, 1),
                    Some('w') => (0, 1),
                    x => panic!("Invalid after n: {:?}", x),
                },
                x => panic!("Invalid dir {}", x),
            };
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
        if !tiles.insert(pos) {
            tiles.remove(&pos);
        }
    }
    pv!(tiles.len());

    for day in 1..=100 {
        let mut new_tiles = tiles.clone();
        let mut seen = tiles.clone();
        for tile in tiles.iter().copied() {
            let mut cnt = 0;
            for other in neigh_iter(tile) {
                if tiles.contains(&other) {
                    cnt += 1;
                } else {
                    // tile is empty
                    if !seen.insert(other) {
                        continue;
                    }
                    if 2 == neigh_iter(other).filter(|p| tiles.contains(p)).count() {
                        new_tiles.insert(other);
                    }
                }
            }
            if cnt == 0 || cnt > 2 {
                new_tiles.remove(&tile);
            }
        }
        tiles = new_tiles;
    }
    pv!(tiles.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut tiles = HashSet::new();

    for line in input.lines() {
        let mut pos = (0isize, 0isize);
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            let dir = match c {
                'e' => (1, 0),
                'w' => (-1, 0),
                's' => match iter.next() {
                    Some('e') => (0, -1),
                    Some('w') => (-1, -1),
                    x => panic!("Invalid after s: {:?}", x),
                },
                'n' => match iter.next() {
                    Some('e') => (1, 1),
                    Some('w') => (0, 1),
                    x => panic!("Invalid after n: {:?}", x),
                },
                x => panic!("Invalid dir {}", x),
            };
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
        if !tiles.insert(pos) {
            tiles.remove(&pos);
        }
    }
    pv!(tiles.len());
}

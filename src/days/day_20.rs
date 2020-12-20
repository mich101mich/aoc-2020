#![allow(clippy::while_let_on_iterator)]

use crate::utils::*;

fn iter_to_num<'a, I: DoubleEndedIterator<Item = &'a bool>>(v: I) -> u16 {
    v.rev().enumerate().fold(
        0,
        |total, (i, current)| if *current { total | (1 << i) } else { total },
    )
}

fn trim_border(array: &mut Vec<Vec<bool>>) {
    array.remove(0);
    array.pop();
    for row in array.iter_mut() {
        row.remove(0);
        row.pop();
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let sea_monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

    let mut hash_monster: HashSet<(usize, usize)> = sea_monster
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            pv!(line);
            line.chars()
                .enumerate()
                .filter(|(x, c)| *c == '#')
                .map(move |(x, c)| (x, y))
        })
        .collect();
    pv!(hash_monster);

    let mut tiles = HashMap::new();

    let mut iter = input.lines();

    while let Some(line) = iter.next() {
        let id = scanf!(line, "Tile {}:", usize);

        let mut array = vec![];
        while let Some(l) = iter.next() {
            if l.is_empty() {
                break;
            }
            array.push(l.chars().map(|c| c == '#').to_vec());
        }

        tiles.insert(id, array);
    }

    let (w, h) = {
        let array = tiles.values().next().unwrap();
        (array[0].len(), array.len())
    };
    assert_eq!(w, h);

    let mut all_sides = HashMap::new();
    for (id, array) in tiles.iter() {
        let sides = [
            (Dir::Up, array[0].clone()),
            (Dir::Right, array.iter().map(|l| l[w - 1]).to_vec()),
            (Dir::Down, array[w - 1].iter().rev().copied().to_vec()),
            (Dir::Left, array.iter().rev().map(|l| l[0]).to_vec()),
        ];

        for (dir, side) in sides.iter() {
            all_sides.insert((*id, *dir), side.clone());
        }
    }

    let mut borders = HashMap::<u16, Vec<(usize, Dir, bool)>>::new();

    for (&id, array) in tiles.iter() {
        for dir in Dir::all() {
            let side = &all_sides[&(id, dir)];
            let real = iter_to_num(side.iter());
            let rev = iter_to_num(side.iter().rev());
            if let Some(ids) = borders.get_mut(&real) {
                ids.push((id, dir, false));
                borders.get_mut(&rev).unwrap().push((id, dir, true));
            } else {
                borders.insert(real, vec![(id, dir, false)]);
                borders.insert(rev, vec![(id, dir, true)]);
            }
        }
    }

    let mut lonely = HashMap::new();
    for ids in borders.values() {
        if ids.len() != 2 {
            *lonely.entry(ids[0].0).or_insert(0) += 1;
        }
    }
    let corners: HashSet<usize> = lonely
        .iter()
        .filter(|(_, v)| **v == 4) // two sides open, 2 flips per side
        .map(|(k, _)| k)
        .copied()
        .collect();
    pv!(corners);

    let size = w - 2;

    let top_left = 1453;
    let mut current_row = 0;

    let (mut left_match, mut top_match, mut final_tiles) = {
        let mut array = tiles[&top_left].clone();

        let left_match = iter_to_num(array.iter().map(|l| &l[w - 1]));
        let top_match = iter_to_num(array[w - 1].iter().rev());
        trim_border(&mut array);
        (left_match, top_match, array)
    };

    let mut remaining: HashSet<usize> = tiles.keys().copied().collect();
    remaining.remove(&top_left);

    let mut row_width = 1;

    while !remaining.is_empty() {
        if let Some((id, mut dir, _)) = borders[&left_match]
            .iter()
            .find(|(id, _, _)| remaining.contains(id))
            .copied()
        {
            println!("Append {}, {:?}", id, dir);

            remaining.remove(&id);
            row_width += 1;

            let mut array = tiles[&id].clone();
            while dir != Dir::Left {
                dir = dir.clockwise();
                array = (0..w)
                    .map(|y| (0..w).map(|x| array[w - x - 1][y]).to_vec())
                    .to_vec();
            }
            let code = iter_to_num(array.iter().map(|l| &l[0]));
            if code != left_match {
                array.reverse();
            }
            left_match = iter_to_num(array.iter().map(|l| &l[w - 1]));
            trim_border(&mut array);
            for (row_in, row_out) in array
                .iter_mut()
                .zip(final_tiles.iter_mut().skip(current_row))
            {
                row_out.append(row_in);
            }
        } else {
            println!("Next row: {}", row_width);
            row_width = 1;

            let border = &borders[&top_match];

            let (id, mut dir, _) = border
                .iter()
                .find(|(id, _, _)| remaining.contains(id))
                .copied()
                .unwrap();

            println!("Starting with {}, {:?}", id, dir);

            remaining.remove(&id);
            let mut array = tiles[&id].clone();
            while dir != Dir::Up {
                dir = dir.clockwise();
                array = (0..w)
                    .map(|y| (0..w).map(|x| array[w - x - 1][y]).to_vec())
                    .to_vec();
            }
            let code = iter_to_num(array[0].iter().rev());
            if code != top_match {
                array.iter_mut().for_each(|r| r.reverse());
            }
            left_match = iter_to_num(array.iter().map(|l| &l[w - 1]));
            top_match = iter_to_num(array[w - 1].iter().rev());

            trim_border(&mut array);
            final_tiles.append(&mut array);
            current_row += size;
        }
    }

    // println!();
    // for (y, row) in final_tiles.iter().enumerate() {
    //     for (x, tile) in row.iter().enumerate() {
    //         print!("{}", if *tile { '#' } else { '.' });
    //     }
    //     println!();
    // }
    // println!();

    let total_size = final_tiles.len();
    assert_eq!(total_size, final_tiles[0].len());

    let monster_width = sea_monster.lines().next().unwrap().chars().count();
    let monster_height = sea_monster.lines().count();
    let mut count = 0;
    for flipping in 0..8 {
        count = 0;
        for (x, y) in (0..(total_size - monster_height))
            .flat_map(|y| (0..(total_size - monster_width)).map(move |x| (x, y)))
        {
            if hash_monster
                .iter()
                .all(|(dx, dy)| final_tiles[y + dy][x + dx])
            {
                println!("{}, {}", x, y);
                count += 1;
            }
        }
        if count != 0 {
            break;
        }
        final_tiles = (0..total_size)
            .map(|y| {
                (0..total_size)
                    .map(|x| final_tiles[total_size - x - 1][y])
                    .to_vec()
            })
            .to_vec();
        if flipping == 3 {
            final_tiles.reverse();
        }
    }
    pv!(count);
    let total = final_tiles
        .iter()
        .flat_map(|r| r.iter())
        .filter(|v| **v)
        .count();
    pv!(total);
    pv!(total - count * hash_monster.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut tiles = HashMap::new();

    let mut iter = input.lines();

    while let Some(line) = iter.next() {
        let id = scanf!(line, "Tile {}:", usize);

        let mut array = vec![];
        while let Some(l) = iter.next() {
            if l.is_empty() {
                break;
            }
            array.push(l.chars().map(|c| c == '#').to_vec());
        }

        tiles.insert(id, array);
    }

    let (w, h) = {
        let array = tiles.values().next().unwrap();
        (array[0].len(), array.len())
    };

    let mut borders = HashMap::<u16, Vec<(usize, Dir, bool)>>::new();

    for (id, array) in tiles.iter() {
        let sides = [
            (Dir::Up, array[0].clone()),
            (
                Dir::Right,
                array.iter().filter_map(|l| l.last()).copied().to_vec(),
            ),
            (Dir::Down, array.last().unwrap().clone()),
            (Dir::Left, array.iter().map(|l| l[0]).to_vec()),
        ];

        for (dir, side) in sides.iter() {
            let real = iter_to_num(side.iter());
            let rev = iter_to_num(side.iter().rev());
            if let Some(ids) = borders.get_mut(&real) {
                ids.push((*id, *dir, false));
            } else if let Some(ids) = borders.get_mut(&rev) {
                ids.push((*id, *dir, true));
            } else {
                borders.insert(real, vec![(*id, *dir, false)]);
            }
        }
    }

    let mut lonely = HashMap::new();
    for ids in borders.values() {
        if ids.len() != 2 {
            pv!(ids);
            *lonely.entry(ids[0].0).or_insert(0) += 1;
        }
    }
    pv!(lonely.iter().filter(|(_, v)| **v == 2).to_vec());
    pv!(lonely
        .iter()
        .filter(|(_, v)| **v == 2)
        .map(|(k, _)| k)
        .product::<usize>());
}

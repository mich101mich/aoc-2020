#![allow(clippy::while_let_on_iterator)]

use crate::utils::*;

fn iter_to_num<'a, I: DoubleEndedIterator<Item = &'a bool>>(v: I) -> u16 {
    v.rev().enumerate().fold(
        0,
        |total, (i, current)| if *current { total | (1 << i) } else { total },
    )
}

fn trim_border(array: &mut Vec<Vec<bool>>) {
    array.pop();
    array.remove(0);
    for row in array.iter_mut() {
        row.pop();
        row.remove(0);
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let sea_monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

    let hash_monster = sea_monster
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(x, c)| *c == '#')
                .map(move |(x, c)| (x, y))
        })
        .to_set();

    let mut tiles = HashMap::new();

    let mut iter = input.lines();

    while let Some(line) = iter.next() {
        let id = scanf!(line, "Tile {}:", usize);

        let mut array = vec![];
        while let Some(l) = iter.next() {
            if l.is_empty() {
                break;
            }
            array.push(hashtag_line(l));
        }

        tiles.insert(id, array);
    }

    let (w, h) = {
        let array = tiles.values().next().unwrap();
        (array[0].len(), array.len())
    };
    assert_eq!(w, h);

    let mut borders = HashMap::<u16, Vec<usize>>::new();
    for (id, array) in tiles.iter() {
        let sides = [
            array[0].clone(),                        // Dir::Up
            array.iter().map(|l| l[w - 1]).to_vec(), // Dir::Right
            array[w - 1].clone(),                    // Dir::Down
            array.iter().map(|l| l[0]).to_vec(),     // Dir::Left
        ];

        for side in sides.iter() {
            let real = iter_to_num(side.iter());
            let rev = iter_to_num(side.iter().rev());
            if let Some(ids) = borders.get_mut(&real) {
                ids.push(*id);
                borders.get_mut(&rev).unwrap().push(*id);
            } else {
                borders.insert(real, vec![*id]);
                borders.insert(rev, vec![*id]);
            }
        }
    }

    let mut lonely = HashMap::new();
    for ids in borders.values() {
        if ids.len() != 2 {
            *lonely.entry(ids[0]).or_insert(0) += 1;
        }
    }
    let corners = lonely
        .iter()
        .filter(|(_, v)| **v == 4) // two sides open, 2 flips per side
        .map(|(k, _)| k)
        .copied()
        .to_set();

    let size = w - 2;

    let top_left = *corners.iter().next().unwrap();
    pv!(top_left);

    let mut current_row = 0;

    let (mut left_match, mut top_match, mut final_tiles) = {
        let mut array = tiles[&top_left].clone();
        let mut left_match = 0;
        let mut top_match = 0;

        for rotating in 0..8 {
            left_match = iter_to_num(array.iter().map(|l| &l[w - 1]));
            top_match = iter_to_num(array[w - 1].iter().rev());

            if borders[&left_match].len() == 2 && borders[&top_match].len() == 2 {
                break;
            }
            rotate_grid_clock(&mut array);
            if rotating == 3 {
                array.reverse();
            }
        }
        trim_border(&mut array);
        (left_match, top_match, array)
    };

    let mut remaining = tiles.keys().copied().to_set();
    remaining.remove(&top_left);

    while !remaining.is_empty() {
        if let Some(id) = borders[&left_match]
            .iter()
            .find(|id| remaining.contains(id))
            .copied()
        {
            remaining.remove(&id);

            let mut array = tiles[&id].clone();
            for rotating in 0..8 {
                let code = iter_to_num(array.iter().map(|l| &l[0]));
                if code == left_match {
                    break;
                }
                rotate_grid_clock(&mut array);
                if rotating == 3 {
                    array.reverse();
                }
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
            let border = &borders[&top_match];

            let id = border
                .iter()
                .find(|id| remaining.contains(id))
                .copied()
                .unwrap();

            remaining.remove(&id);
            let mut array = tiles[&id].clone();
            for rotating in 0..8 {
                let code = iter_to_num(array[0].iter().rev());
                if code == top_match {
                    break;
                }
                rotate_grid_clock(&mut array);
                if rotating == 3 {
                    array.reverse();
                }
            }
            left_match = iter_to_num(array.iter().map(|l| &l[w - 1]));
            top_match = iter_to_num(array[w - 1].iter().rev());

            trim_border(&mut array);
            final_tiles.append(&mut array);
            current_row += size;
        }
    }

    let total_size = final_tiles.len();
    assert_eq!(total_size, final_tiles[0].len());

    let monster_width = sea_monster.lines().next().unwrap().chars().count();
    let monster_height = sea_monster.lines().count();
    let mut count = 0;
    for rotating in 0..8 {
        count = 0;
        for (x, y) in (0..(total_size - monster_height))
            .flat_map(|y| (0..(total_size - monster_width)).map(move |x| (x, y)))
        {
            if hash_monster
                .iter()
                .all(|(dx, dy)| final_tiles[y + dy][x + dx])
            {
                count += 1;
            }
        }
        if count != 0 {
            break;
        }
        rotate_grid_clock(&mut final_tiles);
        if rotating == 3 {
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
    assert_eq!(w, h);

    let mut borders = HashMap::<u16, Vec<(usize, Dir)>>::new();
    for (id, array) in tiles.iter() {
        let sides = [
            (Dir::Up, array[0].clone()),
            (Dir::Right, array.iter().map(|l| l[w - 1]).to_vec()),
            (Dir::Down, array[w - 1].clone()),
            (Dir::Left, array.iter().map(|l| l[0]).to_vec()),
        ];

        for (dir, side) in sides.iter() {
            let obj = (*id, *dir);
            let real = iter_to_num(side.iter());
            let rev = iter_to_num(side.iter().rev());
            if let Some(ids) = borders.get_mut(&real) {
                ids.push(obj);
                borders.get_mut(&rev).unwrap().push(obj);
            } else {
                borders.insert(real, vec![obj]);
                borders.insert(rev, vec![obj]);
            }
        }
    }

    let mut lonely = HashMap::new();
    for ids in borders.values() {
        if ids.len() != 2 {
            *lonely.entry(ids[0].0).or_insert(0) += 1;
        }
    }
    let corners = lonely
        .iter()
        .filter(|(_, v)| **v == 4) // two sides open, 2 flips per side
        .map(|(k, _)| k)
        .product::<usize>();
    pv!(corners);
}

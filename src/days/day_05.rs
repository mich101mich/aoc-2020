use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut ids = HashSet::new();

    for seat in input.lines() {
        let mut front = 0;
        let mut back = 128;
        let mut left = 0;
        let mut right = 8;
        for c in seat.chars() {
            match c {
                'F' => back = (front + back) / 2,
                'B' => front = (front + back) / 2,
                'L' => right = (left + right) / 2,
                'R' => left = (left + right) / 2,
                _ => panic!("invalid char"),
            }
        }
        assert_eq!(front, back - 1);
        assert_eq!(left, right - 1);
        let id = front * 8 + left;
        ids.insert(id);
    }
    for id in 0..1000 {
        if !ids.contains(&id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)) {
            println!("id: {:?}", id);
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut max = 0;

    for seat in input.lines() {
        let mut front = 0;
        let mut back = 128;
        let mut left = 0;
        let mut right = 8;
        for c in seat.chars() {
            match c {
                'F' => back = (front + back) / 2,
                'B' => front = (front + back) / 2,
                'L' => right = (left + right) / 2,
                'R' => left = (left + right) / 2,
                _ => panic!("invalid char"),
            }
        }
        assert_eq!(front, back - 1);
        assert_eq!(left, right - 1);
        let id = front * 8 + left;
        max = max.max(id);
    }
    println!("max: {:?}", max);
}

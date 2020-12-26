use crate::utils::*;

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let parsed = input.lines().map(parse_u).to_vec();
    let card_pub = parsed[0];
    let door_pub = parsed[1];

    let limit = 20201227;
    let mut card_loop_size = 0;
    let mut door_loop_size = 0;

    let mut val = 1;
    while val != card_pub {
        val = (val * 7) % limit;
        card_loop_size += 1;
    }
    pv!(val, card_pub);
    val = 1;
    while val != door_pub {
        val = (val * 7) % limit;
        door_loop_size += 1;
    }
    pv!(val, door_pub);

    val = 1;
    for _ in 0..card_loop_size {
        val = (val * door_pub) % limit;
    }
    pv!(val);
    val = 1;
    for _ in 0..door_loop_size {
        val = (val * card_pub) % limit;
    }
    pv!(val);
}

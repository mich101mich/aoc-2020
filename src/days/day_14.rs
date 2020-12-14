use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = ;

    let mut memory = HashMap::<u64, u64>::new();
    let mut mask_ones: u64 = 0;
    let mut floating = 1;
    let mut floating_bits = vec![];

    for line in input.lines() {
        if let Ok(mask) = scan_fmt!(line, "mask = {}", String) {
            mask_ones = 0;
            floating = 1;
            floating_bits.clear();
            for (i, c) in mask.chars().rev().enumerate() {
                if c == '1' {
                    mask_ones |= 1 << i;
                } else if c == '0' {
                    // nothing
                } else if c == 'X' {
                    floating_bits.push(i);
                    floating *= 2;
                } else {
                    panic!("Invalid char: {}", c);
                }
            }
        } else if let Ok((addr, val)) = scan_fmt!(line, "mem[{}] = {}", u64, u64) {
            let addr = addr | mask_ones;
            for mask in 0..floating {
                let mut current = addr;
                for (i, bit) in floating_bits.iter().enumerate() {
                    if (mask >> i) & 1 == 1 {
                        current |= (1 << bit);
                    } else {
                        current &= !(1 << bit);
                    }
                }
                *memory.entry(current).or_insert(0) = val;
            }
        }
    }
    let sum: u64 = memory.values().sum();
    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = ;

    let mut memory = HashMap::<u64, u64>::new();
    let mut mask_ones: u64 = 0;
    let mut mask_zeros: u64 = 0;

    for line in input.lines() {
        if let Ok(mask) = scan_fmt!(line, "mask = {}", String) {
            mask_ones = 0;
            mask_zeros = 0;
            for (i, c) in mask.chars().rev().enumerate() {
                if c == '1' {
                    mask_ones |= 1 << i;
                    mask_zeros |= 1 << i;
                } else if c == '0' {
                    // nothing
                } else if c == 'X' {
                    mask_zeros |= 1 << i;
                } else {
                    panic!("Invalid char: {}", c);
                }
            }
        } else if let Ok((addr, val)) = scan_fmt!(line, "mem[{}] = {}", u64, u64) {
            let v = (val & mask_zeros) | mask_ones;
            *memory.entry(addr).or_insert(0) = v;
        }
    }
    let sum: u64 = memory.values().sum();
    pv!(sum);
}

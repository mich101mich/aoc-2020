use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");
    // let input = ;

    let parsed = input.lines().map(|l| l.to_owned()).to_vec();
    let len = parsed.len();

    'outer: for i in 0..parsed.len() {
        let nop = parsed[i].starts_with("nop");
        let jmp = parsed[i].starts_with("jmp");
        if !nop && !jmp {
            continue;
        }
        let mut parsed = parsed.clone();
        if nop {
            parsed[i] = format!("jmp {}", parsed[i].split(' ').nth(1).unwrap());
        } else {
            parsed[i] = format!("nop {}", parsed[i].split(' ').nth(1).unwrap());
        }
        let mut acc = 0;
        let mut index = 0;
        loop {
            let line = &mut parsed[index];
            if let Ok((n)) = scan_fmt!(line, "acc {}", i32) {
                acc += n;
                index += 1;
                *line = String::from("stop");
            } else if let Ok((n)) = scan_fmt!(line, "jmp {}", i32) {
                let new_index = index as i32 + n;
                if new_index < 0 || new_index > len as i32 + 1 {
                    continue 'outer;
                }
                index = new_index as usize;
                *line = String::from("stop");
            } else if *line == "stop" {
                continue 'outer;
            } else {
                index += 1;
            }
            if index == len {
                println!("i: {:?}", i);
                println!("acc: {:?}", acc);
                break 'outer;
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");
    // let input = ;

    let mut parsed = input.lines().to_vec();
    let mut acc = 0;
    let mut index = 0;
    loop {
        let line = &mut parsed[index];
        if let Ok((n)) = scan_fmt!(line, "acc {}", i32) {
            acc += n;
            index += 1;
            *line = "stop";
        } else if let Ok((n)) = scan_fmt!(line, "jmp {}", i32) {
            index = (index as i32 + n) as usize;
            *line = "stop";
        } else if *line == "stop" {
            break;
        } else {
            index += 1;
        }
    }
    println!("acc: {:?}", acc);
}

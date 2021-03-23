use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");
    // let input = ;

    let parsed = input.lines().map(|l| l.to_owned()).to_vec();
    let len = parsed.len();

    'outer: for i in 0..parsed.len() {
        let mut parsed = parsed.clone();
        if let Some(v) = parsed[i].strip_prefix("nop ") {
            parsed[i] = format!("jmp {}", v);
        } else if let Some(v) = parsed[i].strip_prefix("jmp ") {
            parsed[i] = format!("nop {}", v);
        } else {
            continue;
        }
        let mut acc = 0;
        let mut index = 0;
        loop {
            let line = &mut parsed[index];
            if let Some(n) = scanf!(line, "acc {}", isize) {
                acc += n;
                index += 1;
                *line = String::from("stop");
            } else if let Some(n) = scanf!(line, "jmp {}", isize) {
                let new_index = index as isize + n;
                if new_index < 0 || new_index > len as isize + 1 {
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
        if let Some(n) = scanf!(line, "acc {}", isize) {
            acc += n;
            index += 1;
            *line = "stop";
        } else if let Some(n) = scanf!(line, "jmp {}", isize) {
            index = (index as isize + n) as usize;
            *line = "stop";
        } else if *line == "stop" {
            break;
        } else {
            index += 1;
        }
    }
    println!("acc: {:?}", acc);
}

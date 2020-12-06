use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut count = 0;
    let mut answers = HashSet::new();
    let mut first = true;

    for line in input.lines() {
        if line.trim().is_empty() {
            count += answers.len();
            answers.clear();
            first = true;
        } else {
            let answer: HashSet<char> = line.trim().chars().collect();
            if first {
                answers = answer;
                first = false;
            } else {
                answers = answers.intersection(&answer).copied().collect();
            }
        }
    }
    count += answers.len();

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut count = 0;
    let mut answers = HashSet::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            count += answers.len();
            answers.clear();
        } else {
            for c in line.trim().chars() {
                answers.insert(c);
            }
        }
    }
    count += answers.len();

    pv!(count);
}

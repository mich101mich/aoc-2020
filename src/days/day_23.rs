use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");
    // let input = "389125467";

    let mut parsed = input.chars().map(|c| c as usize - b'0' as usize).to_vec();

    let max = 1_000_000;
    let mut next = vec![0; max + 1];
    for (a, b) in parsed.iter().zip(parsed.iter().skip(1)) {
        next[*a] = *b;
    }

    let n = parsed.len() + 1;
    next[*parsed.last().unwrap()] = n;
    for (i, v) in next.iter_mut().enumerate().skip(n) {
        *v = i + 1;
    }
    next[max] = parsed[0];

    let mut current = parsed[0];
    let iterations = 10_000_000;
    for i in 0..iterations {
        let a = next[current];
        let b = next[a];
        let c = next[b];
        next[current] = next[c];
        let mut search = current - 1;
        if search == 0 {
            search = max;
        }
        while search == a || search == b || search == c {
            search -= 1;
            if search == 0 {
                search = max;
            }
        }
        next[c] = next[search];
        next[search] = a;

        current = next[current];
    }
    let a = next[1];
    let b = next[a];
    pv!(a, b, a * b);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");
    // let input = "389125467";

    let mut parsed = input
        .chars()
        .map(|c| c as usize - b'0' as usize)
        .collect::<VecDeque<_>>();
    let min = *parsed.iter().min().unwrap();
    let max = *parsed.iter().max().unwrap();

    for _ in 0..100 {
        let mut current = parsed.pop_front().unwrap();
        parsed.push_back(current);
        let a = parsed.pop_front().unwrap();
        let b = parsed.pop_front().unwrap();
        let c = parsed.pop_front().unwrap();
        let dest = loop {
            if current == min {
                current = max;
            } else {
                current -= 1;
            }
            if let Some((pos, _)) = parsed.iter().enumerate().find(|(_, v)| **v == current) {
                break pos;
            }
        };
        parsed.insert(dest + 1, c);
        parsed.insert(dest + 1, b);
        parsed.insert(dest + 1, a);
    }

    while parsed[0] != 1 {
        let mut current = parsed.pop_front().unwrap();
        parsed.push_back(current);
    }
    print_arr!(parsed);
}

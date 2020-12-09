use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input.lines().map(parse).to_vec();

    let mut number = 0;

    const STEP: usize = 25;
    'outer: for (n, range) in parsed
        .iter()
        .skip(STEP)
        .enumerate()
        .map(|(i, n)| (n, &parsed[i..(i + STEP)]))
    {
        for a in range {
            for b in range {
                if a + b == *n {
                    continue 'outer;
                }
            }
        }
        number = *n;
        break;
    }

    'outer2: for start in 0..parsed.len() {
        let mut sum = 0;
        let mut i = start;
        while sum < number {
            sum += parsed[i];
            i += 1;
        }
        if sum == number && i - start >= 2 {
            let range = &parsed[start..i];
            let max = range.iter().max().unwrap();
            let min = range.iter().min().unwrap();
            println!("min + max: {:?}", min + max);
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input.lines().map(parse).to_vec();

    const STEP: usize = 25;
    'outer: for (n, range) in parsed
        .iter()
        .skip(STEP)
        .enumerate()
        .map(|(i, n)| (n, &parsed[i..(i + STEP)]))
    {
        for a in range {
            for b in range {
                if a + b == *n {
                    continue 'outer;
                }
            }
        }
        println!("n: {:?}", n);
    }
}

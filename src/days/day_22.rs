use crate::utils::*;

fn fight(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (bool, usize) {
    let mut seen = HashSet::new();
    loop {
        if p1.is_empty() {
            println!("p2 wins");
            let sum = p2
                .iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) * c)
                .sum::<usize>();
            return (false, sum);
        }
        if p2.is_empty() {
            println!("p1 wins");
            let sum = p1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) * c)
                .sum::<usize>();
            return (true, sum);
        }

        if !seen.insert((p1.clone(), p2.clone())) {
            println!("p1 win by default");
            return (true, 0);
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let p1_wins = if p1.len() >= c1 && p2.len() >= c2 {
            fight(
                p1.iter().take(c1).copied().collect(),
                p2.iter().take(c2).copied().collect(),
            )
            .0
        } else {
            c1 > c2
        };

        if p1_wins {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut p1 = input
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_u)
        .collect::<VecDeque<_>>();
    let mut p2 = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(2)
        .map(parse_u)
        .collect::<VecDeque<_>>();

    let score = fight(p1, p2);
    pv!(score);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");
    // let input = ;

    let mut p1 = input
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_u)
        .collect::<VecDeque<_>>();
    let mut p2 = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(2)
        .map(parse_u)
        .collect::<VecDeque<_>>();

    loop {
        if p1.is_empty() {
            println!("p2 wins");
            let mut sum = p2
                .iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) * c)
                .sum::<usize>();
            pv!(sum);
            break;
        }
        if p2.is_empty() {
            println!("p1 wins");
            let mut sum = p1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, c)| (i + 1) * c)
                .sum::<usize>();
            pv!(sum);
            break;
        }
        let a = p1.pop_front().unwrap();
        let b = p2.pop_front().unwrap();
        if a > b {
            p1.push_back(a);
            p1.push_back(b);
        } else {
            p2.push_back(b);
            p2.push_back(a);
        }
    }
}

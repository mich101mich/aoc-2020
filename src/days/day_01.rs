use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input.lines().map(parse).to_vec();

    for a in &parsed {
        for b in &parsed {
            for c in &parsed {
                if a != b && a != c && b != c && a + b + c == 2020 {
                    pv!(a);
                    pv!(b);
                    pv!(c);
                    pv!(a * b * c);
                    return;
                }
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input.lines().map(parse).to_vec();

    for a in &parsed {
        for b in &parsed {
            if a != b && a + b == 2020 {
                pv!(a);
                pv!(b);
                pv!(a * b);
                return;
            }
        }
    }
}

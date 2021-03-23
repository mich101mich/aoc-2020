use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let (key, values) = scanf!(l, "{} bags contain {}", String, String).unwrap();
            let values = values
                .split(", ")
                .filter_map(|s| scanf_unescaped!(s, "{} {} bags?\\.?", usize, String))
                .to_vec();
            (key, values)
        })
        .to_map();

    let mut cnt = 0;
    let mut next = vec![(1, String::from("shiny gold"))];

    while let Some((amount, name)) = next.pop() {
        cnt += amount;
        for (n, bag) in &parsed[&name] {
            next.push((n * amount, bag.clone()));
        }
    }
    println!("cnt: {:?}", cnt - 1);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");
    //     let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // bright white bags contain 1 shiny gold bag.
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // faded blue bags contain no other bags.
    // dotted black bags contain no other bags.
    // ";
    let parsed = input
        .lines()
        .map(|l| {
            let (key, values) = scanf!(l, "{} bags contain {}", String, String).unwrap();
            let values = values
                .split(", ")
                .filter_map(|s| scanf_unescaped!(s, "{} {} bags?\\.?", usize, String))
                .to_vec();
            (key, values)
        })
        .to_map();

    let mut possible = HashSet::new();

    for (name, bags) in parsed.iter() {
        for (_, bag) in bags.iter() {
            if bag == "shiny gold" {
                possible.insert(name);
            }
        }
    }

    let mut change = true;
    while change {
        change = false;
        for (name, bags) in parsed.iter() {
            for (_, bag) in bags.iter() {
                if possible.contains(bag) && possible.insert(name) {
                    change = true;
                }
            }
        }
    }
    println!("cnt: {:?}", possible.len());
}

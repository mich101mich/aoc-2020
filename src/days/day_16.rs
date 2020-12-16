use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    // let input = ;

    let mut iter = input.lines();
    let mut fields = HashMap::new();

    while let Some(line) = iter.next() {
        if line == "your ticket:" {
            break;
        } else if line.contains(':') {
            let mut s = line.split(": ");
            let key = s.next().unwrap();
            let values = s
                .next()
                .unwrap()
                .split(" or ")
                .map(|r| {
                    let s = r.split('-').map(parse_u).to_vec();
                    s[0]..=s[1]
                })
                .to_vec();
            fields.insert(key.to_string(), values);
        }
    }
    let my_ticket = iter.next().unwrap().split(',').map(parse_u).to_vec();
    iter.next().unwrap();
    iter.next().unwrap();
    let mut tickets = iter.map(|l| l.split(',').map(parse_u).to_vec()).to_vec();

    tickets.retain(|ticket| {
        ticket.iter().all(|val| {
            fields
                .values()
                .flat_map(|v| v.iter())
                .any(|r| r.contains(val))
        })
    });

    let n = fields.len();
    let mut possible = HashMap::new();
    for key in fields.keys() {
        possible.insert(key.clone(), vec![true; n]);
    }
    for ticket in tickets.iter() {
        for (key, ranges) in fields.iter() {
            let mut row = possible.get_mut(key).unwrap();
            for (val, possible) in ticket.iter().zip(row.iter_mut()) {
                if !*possible {
                    continue;
                }
                *possible = ranges.iter().any(|r| r.contains(val));
            }
        }
    }

    for (key, val) in possible.iter() {
        print!("{}: ", key);
        for v in val {
            print!("{}", if *v { '1' } else { '0' });
        }
        println!();
    }
    println!();

    let mut matching = HashMap::new();
    while matching.len() != n {
        let (key, index) = {
            let (k, v) = possible
                .iter()
                .find(|(_, row)| row.iter().filter(|p| **p).count() == 1)
                .unwrap();
            let index = v.iter().enumerate().find(|(_, v)| **v).unwrap().0;
            (k.clone(), index)
        };
        possible.remove(&key);
        for row in possible.values_mut() {
            row[index] = false;
        }
        println!("{}: {}", key, index);
        matching.insert(key, index);
    }
    println!();

    let res: usize = matching
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| my_ticket[*v])
        .product();
    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    // let input = ;

    let mut iter = input.lines();
    let mut fields = HashMap::new();

    while let Some(line) = iter.next() {
        if line == "your ticket:" {
            break;
        } else if line.contains(':') {
            let mut s = line.split(": ");
            let key = s.next().unwrap();
            let values = s
                .next()
                .unwrap()
                .split(" or ")
                .map(|r| {
                    let s = r.split('-').map(parse_u).to_vec();
                    s[0]..=s[1]
                })
                .to_vec();
            fields.insert(key.to_string(), values);
        }
    }
    let my_ticket = iter.next().unwrap().split(',').map(parse_u).to_vec();
    iter.next().unwrap();
    iter.next().unwrap();
    let tickets = iter.map(|l| l.split(',').map(parse_u).to_vec()).to_vec();

    let mut sum = 0;
    for ticket in tickets.iter() {
        for val in ticket {
            let mut valid = fields
                .values()
                .flat_map(|v| v.iter())
                .any(|r| r.contains(val));
            if !valid {
                sum += val;
            }
        }
    }
    pv!(sum);
}

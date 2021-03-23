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
        } else if let Some((key, a1, b1, a2, b2)) = scanf!(
            line,
            "{}: {}-{} or {}-{}",
            String,
            usize,
            usize,
            usize,
            usize
        ) {
            fields.insert(key, vec![a1..=b1, a2..=b2]);
        }
    }
    let my_ticket = comma_values::<usize>(iter.next().unwrap());
    iter.next().unwrap();
    iter.next().unwrap();
    let mut tickets = iter.map(comma_values).to_vec();

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
        matching.insert(key, index);
    }

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
        } else if let Some((key, a1, b1, a2, b2)) = scanf!(
            line,
            "{}: {}-{} or {}-{}",
            String,
            usize,
            usize,
            usize,
            usize
        ) {
            fields.insert(key, vec![a1..=b1, a2..=b2]);
        }
    }
    let my_ticket = comma_values::<usize>(iter.next().unwrap());
    iter.next().unwrap();
    iter.next().unwrap();
    let tickets = iter.map(comma_values).to_vec();

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

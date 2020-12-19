use crate::utils::*;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Sequence(Vec<Vec<usize>>),
}

impl Rule {
    fn to_regex(&self, s: &mut String, rules: &[Rule]) {
        match self {
            Rule::Char(c) => s.push(*c),
            Rule::Sequence(sequences) => {
                s.push('(');
                for seq in sequences.iter() {
                    for rule in seq.iter().map(|i| &rules[*i]) {
                        rule.to_regex(s, rules);
                    }
                    s.push('|');
                }
                s.pop();
                s.push(')');
            }
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");
    // let input = ;

    let first_section = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.split(": ");
            (parse_u(s.next().unwrap()), s.next().unwrap())
        })
        .to_vec();
    let n = first_section.iter().max_by_key(|(n, _)| n).unwrap().0 + 1;

    let mut rules = vec![Rule::Char('x'); n];
    for (k, v) in first_section {
        rules[k] = if let Ok((c)) = scan_fmt!(v, "\"{}\"", char) {
            Rule::Char(c)
        } else {
            Rule::Sequence(
                v.split(" | ")
                    .map(|s| s.split_whitespace().map(parse_u).to_vec())
                    .to_vec(),
            )
        }
    }

    rules[8] = Rule::Sequence(vec![vec![42], vec![42, 8]]);
    rules[11] = Rule::Sequence(vec![vec![42, 31], vec![42, 11, 31]]);

    let mut regexes = vec![None; n];
    let mut visited = vec![false; n];
    let mut change = true;
    while change {
        change = false;
        for (i, rule) in rules.iter().enumerate() {
            if visited[i] {
                continue;
            }
            if i == 8 {
                if !visited[42] {
                    continue;
                }
                regexes[i] = Some(format!("({})+", regexes[42].as_ref().unwrap()));
            } else if i == 11 {
                continue;
            } else {
                match rule {
                    Rule::Char(c) => {
                        regexes[i] = Some(String::from(*c));
                    }
                    Rule::Sequence(sequences) => {
                        if sequences
                            .iter()
                            .flat_map(|s| s.iter())
                            .any(|s| !visited[*s])
                        {
                            continue;
                        }
                        let mut s = String::new();
                        s.push('(');
                        for seq in sequences.iter() {
                            for rule in seq.iter() {
                                s.push_str(regexes[*rule].as_ref().unwrap());
                            }
                            s.push('|');
                        }
                        s.pop();
                        s.push(')');
                        regexes[i] = Some(s);
                    }
                }
            }
            visited[i] = true;
            change = true;
        }
    }

    let mut count = 0;
    let mut unmatched = input.lines().skip_while(|l| !l.is_empty()).to_vec();
    for i in 1..20 {
        // THIS IS THE HACKIEST HACK IN THE HISTORY OF HACKS!!!
        // HOW IS THIS ACTUALLY WORKING???????????
        let mut regex = format!(
            "^({ft}){{{ft_count},}}({ti}){{{ti_count}}}$",
            ft = regexes[42].as_ref().unwrap(),
            ft_count = i + 1,
            ti = regexes[31].as_ref().unwrap(),
            ti_count = i
        );
        let regex = ::regex::Regex::new(&regex).unwrap();
        let mut remaining = vec![];
        for line in unmatched {
            if regex.is_match(line) {
                count += 1;
            } else {
                remaining.push(line);
            }
        }
        unmatched = remaining;
    }

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");
    // let input = ;

    let first_section = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.split(": ");
            (parse_u(s.next().unwrap()), s.next().unwrap())
        })
        .to_vec();
    let n = first_section.iter().max_by_key(|(n, _)| n).unwrap().0 + 1;

    let mut rules = vec![Rule::Char('x'); n];
    for (k, v) in first_section {
        rules[k] = if let Ok((c)) = scan_fmt!(v, "\"{}\"", char) {
            Rule::Char(c)
        } else {
            Rule::Sequence(
                v.split(" | ")
                    .map(|s| s.split_whitespace().map(parse_u).to_vec())
                    .to_vec(),
            )
        }
    }
    let mut regex = String::new();
    regex.push('^');
    rules[0].to_regex(&mut regex, &rules);
    regex.push('$');

    let regex = ::regex::Regex::new(&regex).unwrap();
    let count = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .filter(|l| regex.is_match(l))
        .count();

    pv!(count);
}

use crate::utils::*;

fn in_range(s: &str, low: u32, high: u32) -> Option<()> {
    let byr = s.parse::<u32>().ok()?;
    if byr < low || byr > high {
        return None;
    }
    Some(())
}

fn is_valid(data: &HashMap<&str, &str>) -> Option<()> {
    if data.len() < 7 {
        return None;
    }
    if data.len() == 7 && data.contains_key("cid") {
        return None;
    }
    in_range(data.get("byr")?, 1920, 2002)?;
    in_range(data.get("iyr")?, 2010, 2020)?;
    in_range(data.get("eyr")?, 2020, 2030)?;
    let hgt = data.get("hgt")?;
    if let Some(hgt) = hgt.strip_suffix("cm") {
        in_range(hgt, 150, 193)?;
    } else if let Some(hgt) = hgt.strip_suffix("in") {
        in_range(hgt, 59, 76)?;
    } else {
        return None;
    }
    let hcl = data.get("hcl")?;
    if !hcl.starts_with('#') || hcl.len() != 7 {
        return None;
    }
    u32::from_str_radix(&hcl[1..], 16).ok()?;
    let ecl: HashSet<_> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .collect();
    if !ecl.contains(data.get("ecl")?) {
        return None;
    }
    let pid = data.get("pid")?;
    if pid.len() != 9 {
        return None;
    }
    pid.parse::<u32>().ok()?;

    Some(())
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut current = HashMap::new();
    let mut valid = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            if is_valid(&current).is_some() {
                valid += 1;
            }
            current.clear();
        } else {
            for kv in line.split(' ') {
                let parts = kv.split(':').to_vec();
                current.insert(parts[0], parts[1]);
            }
        }
    }
    if is_valid(&current).is_some() {
        valid += 1;
    }

    pv!(valid);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut current = HashSet::new();
    let mut valid = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            if current.len() == 8 || (current.len() == 7 && !current.contains("cid")) {
                println!("current: {:?}", current);
                valid += 1;
            }
            current.clear();
        } else {
            for kv in line.split(' ') {
                if !current.insert(kv.split(':').next().unwrap()) {
                    println!("duplicate");
                }
            }
        }
    }
    if current.len() == 8 || (current.len() == 7 && !current.contains("cid")) {
        println!("current: {:?}", current);
        valid += 1;
    }

    pv!(valid);
}

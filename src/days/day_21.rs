use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");
    // let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    // trh fvjkl sbzzf mxmxvkd (contains dairy)
    // sqjhc fvjkl (contains soy)
    // sqjhc mxmxvkd sbzzf (contains fish)";

    let parsed = input
        .lines()
        .map(|l| {
            let mut iter = l.split(" (contains ");
            let ingredients = iter.next().unwrap().split(' ').to_set();
            let allergens = iter
                .next()
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ")
                .to_set();
            (ingredients, allergens)
        })
        .to_vec();

    let mut allergens = parsed.iter().flat_map(|s| s.1.iter()).copied().to_set();

    let mut get_ingredient = HashMap::<&str, &str>::new();
    let mut get_allergen = HashMap::<&str, &str>::new();

    while !allergens.is_empty() {
        let mut leftover = allergens.clone();
        for allergen in allergens.drain() {
            let mut possible: Option<HashSet<&str>> = None;
            for (ing, all) in parsed.iter() {
                if !all.contains(allergen) {
                    continue;
                }
                if let Some(poss) = possible.as_mut() {
                    *poss = poss.intersection(ing).copied().to_set();
                } else {
                    possible = Some(ing.clone());
                }
            }
            if let Some(mut possible) = possible {
                possible.retain(|all| !get_ingredient.values().any(|m| m == all));
                if possible.len() == 1 {
                    let ingredient = possible.into_iter().next().unwrap();
                    get_ingredient.insert(allergen, ingredient);
                    get_allergen.insert(ingredient, allergen);
                    leftover.remove(allergen);
                }
            }
        }
        allergens = leftover;
    }

    let count = parsed
        .iter()
        .flat_map(|(ing, _)| ing.iter())
        .filter(|ing| !get_allergen.contains_key(*ing))
        .count();

    let mut dangerous = get_allergen.keys().copied().to_vec();
    dangerous.sort_by_key(|ing| get_allergen[ing]);
    pv!(dangerous.join(","));
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");
    // let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    // trh fvjkl sbzzf mxmxvkd (contains dairy)
    // sqjhc fvjkl (contains soy)
    // sqjhc mxmxvkd sbzzf (contains fish)";

    let parsed = input
        .lines()
        .map(|l| {
            let mut iter = l.split(" (contains ");
            let ingredients = iter.next().unwrap().split(' ').to_set();
            let allergens = iter
                .next()
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ")
                .to_set();
            (ingredients, allergens)
        })
        .to_vec();

    let mut allergens = parsed.iter().flat_map(|s| s.1.iter()).copied().to_set();

    let mut get_ingredient = HashMap::<&str, &str>::new();
    let mut get_allergen = HashMap::<&str, &str>::new();

    while !allergens.is_empty() {
        let mut leftover = allergens.clone();
        for allergen in allergens.drain() {
            let mut possible: Option<HashSet<&str>> = None;
            for (ing, all) in parsed.iter() {
                if !all.contains(allergen) {
                    continue;
                }
                if let Some(poss) = possible.as_mut() {
                    *poss = poss.intersection(ing).copied().to_set();
                } else {
                    possible = Some(ing.clone());
                }
            }
            if let Some(mut possible) = possible {
                possible.retain(|all| !get_ingredient.values().any(|m| m == all));
                if possible.len() == 1 {
                    let ingredient = possible.into_iter().next().unwrap();
                    get_ingredient.insert(allergen, ingredient);
                    get_allergen.insert(ingredient, allergen);
                    leftover.remove(allergen);
                }
            }
        }
        allergens = leftover;
    }

    let count = parsed
        .iter()
        .flat_map(|(ing, _)| ing.iter())
        .filter(|ing| !get_allergen.contains_key(*ing))
        .count();
    pv!(count);
}

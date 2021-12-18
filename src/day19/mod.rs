use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Rules = HashMap<String, HashSet<String>>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (rules, initial_str) = parse_input(&input);
    let result = apply(&initial_str, &rules);
    println!("{:?}", result);
    result.len() as i128
}

fn parse_input(input: &str) -> (Rules, &str) {
    let (rules, initial_str) = input.split("\n\n")
        .tuples::<(&str, &str)>()
        .next()
        .unwrap();

    let tuples: Vec<_> = rules.split_whitespace()
        .tuples::<(&str, &str, &str)>()
        .map(|(k, _, v)| (k.to_string(), v.to_string()))
        .collect();

    let mut rules = hashmap!{};
    for (key, group) in &tuples.into_iter().group_by(|(k, _)| k.clone()) {
        rules.insert(key, group.map(|(_, v)| v).collect());
    }

    (rules, initial_str)
}

fn apply(input: &str, rules: &Rules) -> HashSet<String> {
    let mut result = HashSet::new();
    for (key, replacements) in rules.iter() {
        let set: HashSet<_> = find_substring(input, key).iter()
            .flat_map(|i| {
                replacements.iter()
                    .map(|rep| {
                        let mut new = input.to_string();
                        new.replace_range(*i..(*i + key.len()), rep);
                        new
                    })
            })
            .collect();
        result.extend(set);
    }

    result
}

fn find_substring(input: &str, substr: &str) -> Vec<usize> {
    let mut i = 0;
    let mut result = Vec::new();
    while let Some(s) = input[i..].find(substr) {
        result.push(i + s);
        i += s + 1;
    }
    result
}

fn star2(input: String) -> i128 {
    0
}

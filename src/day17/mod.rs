use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::HashMap;

type Cache = HashMap<(i128, Set), HashSet<Set>>;
type Set = BTreeSet<(usize, i128)>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    combinations(&input).len() as i128
}

fn combinations(input: &str) -> HashSet<Set> {
    let c: Vec<(usize, i128)> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();
    let mut cache = Cache::new();
    fit(c, 150, Set::new(), &mut cache)
}

fn fit(
    c: Vec<(usize, i128)>, 
    n: i128, 
    path: Set, 
    cache: &mut Cache,
) -> HashSet<Set> {
    if  n == 0 {
        let mut result = HashSet::new();
        result.insert(path.clone());
        return result;
    }

    let key = (n, path.clone());
    if cache.contains_key(&key) {
        return cache[&key].clone();
    }

    c.iter()
        .filter(|(_, v)| v <= &n)
        .map(|(i, v)| {
            let c_: Vec<_> = c.iter()
                .filter(|(j, _)| i != j)
                .cloned()
                .collect();
            let n_ = n - v;
            let mut path_ = path.clone();
            path_.insert((*i, *v));
            let result = fit(c_, n_, path_, cache);
            cache.insert(key.clone(), result.clone());
            result
        })
        .fold(HashSet::new(), |a, b| a.union(&b).cloned().collect())
}

fn star2(input: String) -> i128 {
    let result = combinations(&input);

    let m = result.iter()
        .map(|s| s.len())
        .min()
        .unwrap();

    result.iter()
        .filter(|s| s.len() == m)
        .count() as i128
}

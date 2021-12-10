use itertools::Itertools;
#[macro_use] extern crate maplit;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    get_combinations(&input).len() as i128
}

fn get_combinations(input: &str) -> Vec<Vec<i128>> {
    let c: Vec<i128> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (1..=c.len())
        .flat_map(|i| 
            c.iter().combinations(i)
                .map(|c| 
                    c.into_iter()
                        .copied()
                        .collect::<Vec<_>>()
                )
        )
        .filter(|s| s.iter().sum::<i128>() == 150)
        .collect()
}

fn star2(input: String) -> i128 {
    let result = get_combinations(&input);
    let min = result.iter().map(|p| p.len()).min().unwrap();
    result.iter()
        .filter(|p| p.len() == min)
        .count() as i128
}

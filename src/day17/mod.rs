use itertools::Itertools;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let c: Vec<i128> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (1..=c.len())
        .flat_map(|i| 
            c.iter().combinations(i)
                .map(|c| c.iter().copied().sum::<i128>())
        )
        .filter(|&s| s == 150)
        .count() as i128
}

fn star2(input: String) -> i128 {
    let c: Vec<i128> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result: Vec<Vec<i128>> = (1..=c.len())
        .flat_map(|i| 
            c.iter().combinations(i)
                .map(|c| 
                    c.into_iter()
                        .copied()
                        .collect::<Vec<_>>()
                )
        )
        .filter(|s| s.iter().sum::<i128>() == 150)
        .collect();

    let m = result.iter()
        .map(|p| p.len())
        .min()
        .unwrap();

    result.iter()
        .filter(|p| p.len() == m)
        .count() as i128
}

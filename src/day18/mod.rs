use std::collections::HashMap;

type Pos = (i128, i128);
type Map = HashMap<Pos, bool>;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let (mut map, _) = parse_input(&input);

    for _ in 0..100 {
        map = evolve(map);
    }

    map.values()
        .filter(|&on| *on)
        .count() as i128
}

fn parse_input(input: &str) -> (Map, i128) {
    let n = input.split_whitespace().count();
    let map = input.split_whitespace()
        .enumerate()
        .flat_map(|(y, line)| 
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i128, y as i128), c == '#'))
        )
        .collect();

    (map, n as i128)
}

fn evolve(map: Map) -> Map {
    map.iter()
        .map(|(pos, &on)| {
            let n = neighbours(&map, *pos);
            (*pos, (on && [2, 3].contains(&n)) || (!on && n == 3))
        })
        .collect()
}

fn neighbours(map: &Map, pos: Pos) -> i128 {
    let (x, y) = pos;
    ((y - 1)..=(y + 1))
        .flat_map(|y_| 
            ((x - 1)..=(x + 1))
                .map(move |x_| (x_, y_))
        )
        .filter(|(x_, y_)| *x_ != x || *y_ != y)
        .filter(|pos| map.contains_key(pos) && map[pos])
        .count() as i128
}

fn star2(input: String) -> i128 {
    let (mut map, dim) = parse_input(&input);

    for _ in 0..100 {
        map = evolve2(map, dim);
    }

    map.values()
        .filter(|&on| *on)
        .count() as i128
}

fn evolve2(map: Map, dim: i128) -> Map {
    map.iter()
        .map(|(pos, &on)| {
            let n = neighbours2(&map, *pos, dim);
            (
                *pos, 
                [(0, 0), (0, dim - 1), (dim - 1, 0), (dim - 1, dim - 1)].contains(pos) ||
                (on && [2, 3].contains(&n)) ||
                (!on && n == 3)
            )
        })
        .collect()
}

fn neighbours2(map: &Map, pos: Pos, dim: i128) -> i128 {
    let (x, y) = pos;
    ((y - 1)..=(y + 1))
        .flat_map(|y_| 
            ((x - 1)..=(x + 1))
                .map(move |x_| (x_, y_))
        )
        .filter(|(x_, y_)| *x_ != x || *y_ != y)
        .filter(|pos| [(0, 0), (0, dim - 1), (dim - 1, 0), (dim - 1, dim - 1)].contains(pos) || (map.contains_key(pos) && map[pos]))
        .count() as i128
}

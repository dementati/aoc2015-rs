use std::collections::HashMap;

use regex::Regex;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

fn star1(input: String) -> i128 {
    let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    let mut data = HashMap::new();
    let lines: Vec<_> = input.split("\n").collect();
    for line in lines.into_iter() {
        for cap in re.captures_iter(&line) {
            let mut sue = HashMap::new();
            sue.insert(cap[2].to_string(), *&cap[3].parse::<i128>().unwrap());
            sue.insert(cap[4].to_string(), *&cap[5].parse::<i128>().unwrap());
            sue.insert(cap[6].to_string(), *&cap[7].parse::<i128>().unwrap());
            data.insert(*&cap[1].parse::<i128>().unwrap(), sue);
        }
    }

    for (id, sue) in data.into_iter() {
        let found = sue.iter()
            .all(|(key, &value)| match key.as_str() {
                "children" => value == 3,
                "cats" => value == 7,
                "samoyeds" => value == 2,
                "pomeranians" => value == 3,
                "akitas" => value == 0,
                "vizslas" => value == 0,
                "goldfish" => value == 5,
                "trees" => value == 3,
                "cars" => value == 2,
                "perfumes" => value == 1,
                _ => panic!("Unknown key"),
            });

        if found {
            return id;
        }
    }

    panic!("Sue not found!");
}

fn star2(input: String) -> i128 {
    let re = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    let mut data = HashMap::new();
    let lines: Vec<_> = input.split("\n").collect();
    for line in lines.into_iter() {
        for cap in re.captures_iter(&line) {
            let mut sue = HashMap::new();
            sue.insert(cap[2].to_string(), *&cap[3].parse::<i128>().unwrap());
            sue.insert(cap[4].to_string(), *&cap[5].parse::<i128>().unwrap());
            sue.insert(cap[6].to_string(), *&cap[7].parse::<i128>().unwrap());
            data.insert(*&cap[1].parse::<i128>().unwrap(), sue);
        }
    }

    for (id, sue) in data.into_iter() {
        let found = sue.iter()
            .all(|(key, &value)| match key.as_str() {
                "children" => value == 3,
                "cats" => value > 7,
                "samoyeds" => value == 2,
                "pomeranians" => value < 3,
                "akitas" => value == 0,
                "vizslas" => value == 0,
                "goldfish" => value < 5,
                "trees" => value > 3,
                "cars" => value == 2,
                "perfumes" => value == 1,
                _ => panic!("Unknown key"),
            });

        if found {
            return id;
        }
    }

    panic!("Sue not found!");
}

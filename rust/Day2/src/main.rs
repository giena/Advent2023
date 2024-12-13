use std::collections::HashMap;
use std::fs::read_to_string;

fn possible(game: &Vec<(&str, u32)>, weight: u32, maxs: &HashMap<&str, u32>) -> u32 {
    let mut result: u32 = weight;
    for pair in game {
        if pair.1 > *maxs.get(pair.0).or(Some(&std::u32::MAX)).unwrap() {
            result = 0;
        }
    }
    println!("Game {} = {}", weight, result);
    result
}

fn power(game: &Vec<(&str, u32)>) -> u32 {
    let mut cubes: HashMap<&str, u32> = HashMap::new();
    for pair in game {
        if pair.1 > *cubes.get(pair.0).or(Some(&0)).unwrap() {
            cubes.insert(pair.0, pair.1);
        }
    }
    let mut numbers: u32 = 0;
    for (_, number) in cubes {
        if numbers == 0 {
            numbers = number;
        } else {
            numbers = numbers * number;
        }
    }
    numbers
}

fn main() {
    let maxs: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut result1: u32 = 0;
    let mut result2: u32 = 0;

    for line in read_to_string("./input").unwrap().lines() {
        let splits: Vec<&str> = line[5..].split(&[':', ',', ';']).collect();
        let weight = splits[0].parse().unwrap();
        let mut game: Vec<(&str, u32)> = vec![];
        for s in 1..splits.len() {
            let splits: Vec<&str> = splits[s].trim().split(' ').collect();
            game.push((splits[1], splits[0].parse().unwrap()));
        }
        result1 += possible(&game, weight, &maxs);
        result2 += power(&game);
    }

    println!("{}", result1);
    println!("{}", result2);
}

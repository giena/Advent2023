use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let mut hand_results: HashMap<HandResult, Vec<(&str, String, u32, u64)>> = HashMap::new();
    let mut set_results: HashSet<HandResult> = HashSet::new();

    let binding = read_to_string("./input2").unwrap();
    let mut num_games = 0;
    for line in binding.lines() {
        num_games = num_games + 1;
        let split: Vec<&str> = line
            .trim()
            .split([' '])
            .filter(|s| s.trim().len() > 0)
            .collect();
        let weight = weight(split[0]);
        let best = compute_best(split[0]);
        let r = hand_result(best.as_str());
        let winning: u32 = split[1].parse().unwrap();
        let tuple: (&str, String, u32, u64) = (split[0], best, winning, weight);
        let vec = hand_results.entry(r.clone()).or_insert(vec![]);
        match vec.binary_search_by(|t| t.3.cmp(&tuple.3)) {
            Ok(pos) => {
                println!("bsearch {} pos for {:?}", pos, tuple);
                vec.insert(pos + 1, tuple)
            }
            Err(pos) => vec.insert(pos, tuple),
        }
        set_results.insert(r);
    }

    println!("Number of games = {:?}", num_games);
    num_games = 0;
    let mut keys: Vec<&HandResult> = set_results.iter().collect();
    keys.sort_by(|a, b| a.cmp(b).reverse());

    println!("{:?}", keys);
    for ele in hand_results.values() {
        num_games = num_games + ele.len();
    }
    println!("Number of games = {:?}", num_games);
    //TODO there is only 999 elems. It should be 1000.

    let mut result: u32 = 0;

    let mut rank: u32 = 1;
    for i in 0..keys.len() {
        let v = hand_results.entry(keys[i].clone()).or_insert(vec![]);
        for ele in v {
            result = result + (ele.2 * rank);
            println!(
                "{:?} {}/{} => ({},{}) with rank {} = {}",
                keys[i].clone(),
                ele.0,
                ele.1,
                ele.2,
                ele.3,
                rank,
                result
            );
            rank = rank + 1;
        }
    }

    println!("{}", result);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
enum HandResult {
    Five,
    Square,
    Full,
    Brelan,
    TwoPaires,
    OnePaire,
    High,
}

fn compute_best(game: &str) -> String {
    let mut map: HashMap<char, u32> = HashMap::new();
    for char in game.chars() {
        if char != 'J' {
            let mut value = map.get(&char);
            let value = &value.get_or_insert(&0);
            map.insert(char, **value + 1);
        }
    }

    let numj = game.chars().filter(|p| *p == 'J').count();

    let popular = map
        .iter()
        .find(|p| {
            let keys = map.keys();
            let values = map.values();
            match values.max() {
                None => panic!("Illegal state"),
                Some(&1) => Some(value_card(*p.0)) == keys.map(|c| value_card(*c)).max(),
                Some(max) => Some(p.1) == Some(max),
            }
        })
        .map(|p| p.0)
        .or(Some(&'A'))
        .unwrap();

    let r = match numj {
        0 => game.to_owned(),
        _ => {
            let mut result = String::with_capacity(game.len());
            for c in game.chars() {
                let next_char = match c {
                    'J' => popular,
                    _ => &c,
                };
                result.push(*next_char);
            }
            result
        }
    };
    println!("Compute best for {} = {}", game, r);
    r
}

fn hand_result(game: &str) -> HandResult {
    let mut map: HashMap<char, u32> = HashMap::new();
    let game = compute_best(game);
    for char in game.chars() {
        let mut value = map.get(&char);
        let value = &value.get_or_insert(&0);
        map.insert(char, **value + 1);
    }

    let values = map.values();
    let max = values.max();
    match map.len() {
        0 => panic!("There must be 5 cards"),
        1 => HandResult::Five,
        2 => match max {
            Some(4) => HandResult::Square,
            Some(3) => HandResult::Full,
            _ => panic!("There must be 5 cards"),
        },
        3 => match max {
            Some(3) => HandResult::Brelan,
            Some(2) => HandResult::TwoPaires,
            _ => panic!("There must be 5 cards"),
        },
        4 => match max {
            Some(2) => HandResult::OnePaire,
            _ => panic!("There must be 5 cards"),
        },
        5 => HandResult::High,
        _ => panic!("There must be 5 cards"),
    }
}

fn value_card(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        digit => digit as u32 - 48,
    }
}

fn weight(game: &str) -> u64 {
    let mut weight: Vec<char> = vec![];
    let chars: Vec<char> = game.chars().collect();
    println!("chars={:?}", chars);
    for pos in 0..chars.len() {
        let num: u32 = value_card(chars[pos]);
        let fchars: Vec<char> = format!("{:0>2}", num).chars().collect();
        for c in fchars {
            weight.push(c);
        }
    }
    let str = String::from_iter(weight);
    println!("str={}", str);
    str.parse().unwrap()
}

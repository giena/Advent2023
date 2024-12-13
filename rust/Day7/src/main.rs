use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let mut hand_results: HashMap<HandResult, Vec<(&str, u32, u64)>> = HashMap::new();
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
        let r = hand_result(split[0]);
        let winning: u32 = split[1].parse().unwrap();
        let weight = weight(split[0]);
        let tuple: (&str, u32, u64) = (split[0], winning, weight);
        let vec = hand_results.entry(r.clone()).or_insert(vec![]);
        match vec.binary_search_by(|t| t.2.cmp(&tuple.2)) {
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
            result = result + (ele.1 * rank);
            println!(
                "{:?} {} => ({},{}) with rank {} = {}",
                keys[i].clone(),
                ele.0,
                ele.1,
                ele.2,
                rank,
                result
            );
            rank = rank + 1;
        }
    }

    println!("{}", result);

    /*
        OnePaire 2942J => (651,29431) with rank 201 = 10334158
    OnePaire 28ATT => (686,29510) with rank 202 = 10472730 */
    println!("{}", weight("2942J"));
    println!("{}", weight("28ATT"));
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

fn hand_result(game: &str) -> HandResult {
    let mut map: HashMap<char, u32> = HashMap::new();
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

fn weight(game: &str) -> u64 {
    let mut weight: Vec<char> = vec![];
    let chars: Vec<char> = game.chars().collect();
    println!("chars={:?}", chars);
    for pos in 0..chars.len() {
        let num: u32 = match chars[pos] {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            digit => digit as u32 - 48,
        };
        let fchars: Vec<char> = format!("{:0>2}", num).chars().collect();
        for c in fchars {
            weight.push(c);
        }
    }
    let str = String::from_iter(weight);
    println!("str={}", str);
    str.parse().unwrap()
}

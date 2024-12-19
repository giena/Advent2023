use find_all::FindAll;
use intbits::Bits;
use std::fs::read_to_string;
use std::usize;

fn combinations(values: &mut Vec<char>, combis: Vec<u32>) -> u32 {
    let mut cumul = 0;
    println!("Values {:?}", values);
    let asks = values.iter().find_all(|c| **c == '?').unwrap();
    println!("Num asks {}", asks.len());
    let num_combis = 2_u32.pow(asks.len() as u32);
    println!("Num combis {}", num_combis);
    for cpt in 0..(num_combis) as usize {
        for (i, index_ask) in asks.iter().enumerate() {
            let val = if cpt.bit(i) { '#' } else { '.' };
            values[*index_ask] = val;
        }
        let mut computed_combis: Vec<u32> = vec![];
        let first = values.iter().position(|c| *c == '#');
        match first {
            Some(first) => {
                let mut cpt_sharp: u32 = 0_u32;
                for v in first..values.len() {
                    match values[v] {
                        '#' => cpt_sharp = cpt_sharp + 1,
                        '.' => {
                            if cpt_sharp != 0 {
                                computed_combis.push(cpt_sharp);
                                cpt_sharp = 0;
                            }
                        }
                        _ => {}
                    }
                }
                if cpt_sharp != 0 {
                    computed_combis.push(cpt_sharp);
                }
                println!("{:?} -> {:?}", values, computed_combis);
                if computed_combis == combis {
                    println!("{:?} == {:?}", computed_combis, combis);
                    cumul = cumul + 1;
                }
            }
            None => {}
        }
    }
    cumul
}

fn main() {
    let mut cumul = 0_u32;
    let binding = read_to_string("./input2").unwrap();

    for line in binding.lines() {
        let mut iter = line.split_ascii_whitespace();
        match iter.next() {
            Some(values_str) => match iter.next() {
                Some(combis_str) => {
                    let mut values: Vec<char> = values_str.chars().collect();
                    let combis: Vec<u32> =
                        combis_str.split(",").map(|v| v.parse().unwrap()).collect();
                    cumul = cumul + combinations(&mut values, combis);
                }
                None => {}
            },
            None => {}
        }
    }

    println!("Cumul {}", cumul);
}

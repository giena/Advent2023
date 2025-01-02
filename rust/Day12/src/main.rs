use find_all::FindAll;
use intbits::Bits;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::repeat;
use std::usize;

fn recur_solve(
    values: &[char],
    combis: &[usize],
    memo: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    let mut total: usize = 0;

    //Here we don't have to call recursive
    if combis.is_empty() {
        if !values.contains(&'#') {
            //values is empty, combis is empty, it's OK.
            return 1;
        }
        return 0; //Not OK, because combis is empty but it remains values
    }

    if let Some(r) = memo.get(&(values.to_vec(), combis.to_vec())) {
        return *r; //Return cached value id already computed
    }

    let minimum_remaining_length = combis.iter().sum::<usize>() + combis.len() - 1;
    if (values.len()) < minimum_remaining_length {
        return 0;
    }

    if &values[0..1] == ['.'] {
        return recur_solve(&values[1..], combis, memo);
    }

    let cur_combi = combis[0];

    let all_values_valid = values[0..cur_combi].iter().all(|c| c != &'.');

    let last_char_valid =
        values.len() == cur_combi || values[cur_combi..cur_combi + 1].iter().all(|c| c != &'#');

    if all_values_valid && last_char_valid {
        let max_idx = values.len().min(cur_combi + 1);
        total += recur_solve(&values[max_idx..], &combis[1..], memo);
    }

    if &values[0..1] != ['#'] {
        total += recur_solve(&values[1..], combis, memo);
    }

    memo.insert((values.to_vec(), combis.to_vec()), total);

    total
}

fn combinations_part1(values: &mut Vec<char>, combis: &Vec<usize>) -> usize {
    let mut cumul = 0;
    println!("Values {:?} with combis {:?}", values, combis);
    let asks = values.iter().find_all(|c| **c == '?').unwrap();
    println!("Num asks {}", asks.len());
    //Number of combinaisons
    //
    let num_combis: u64 = 2_u64.pow(asks.len() as u32);
    println!("Num combis {}", num_combis);
    for cpt in 0..(num_combis) as usize {
        for (i, index_ask) in asks.iter().enumerate() {
            let val = if cpt.bit(i) { '#' } else { '.' };
            values[*index_ask] = val;
        }
        let mut computed_combis: Vec<usize> = vec![];
        let first = values.iter().position(|c| *c == '#');
        match first {
            Some(first) => {
                let mut cpt_sharp: usize = 0_usize;
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
                if computed_combis == *combis {
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
    let mut cumul1: u64 = 0_u64;
    let mut cumul2 = 0_u64;
    let binding = read_to_string("/home/giena/dev/work/advent/2023/rust/Day12/input").unwrap();
    let coeff: usize = 5;
    for line in binding.lines() {
        let mut iter = line.split_ascii_whitespace();
        match iter.next() {
            Some(values_str) => match iter.next() {
                Some(combis_str) => {
                    let mut values: Vec<char> = if coeff == 5 {
                        repeat(values_str)
                            .take(coeff)
                            .collect::<Vec<_>>()
                            .join("?")
                            .chars()
                            .collect()
                    } else {
                        values_str.chars().collect()
                    };
                    let combis: Vec<usize> = combis_str
                        .split(",")
                        .map(|v| v.parse().unwrap())
                        .collect::<Vec<_>>()
                        .repeat(coeff);

                    //cumul1 = cumul1 + combinations_part1(&mut values, &combis) as u64;
                    cumul2 = cumul2
                        + recur_solve(&values[0..], &combis[0..], &mut HashMap::new()) as u64;
                }
                None => {}
            },
            None => {}
        }
    }

    println!("Cumul part 1 {}", cumul1);
    println!("Cumul part 2 {}", cumul2);
}

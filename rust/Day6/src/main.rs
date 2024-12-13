use std::fs::read_to_string;

fn main() {
    let mut entries: Vec<Vec<u64>> = vec![];
    let mut curses_num: usize = 0;
    let fuse = true;
    let quadra = true;
    for line in read_to_string("./input").unwrap().lines() {
        let split: Vec<&str> = line
            .trim()
            .split([' ', ':'])
            .filter(|s| s.trim().len() > 0)
            .collect();
        if fuse {
            let join = split[1..].join("");
            println!("join = {}", join);
            curses_num = 2;
            entries.push(vec![join.parse().unwrap()]);
        } else {
            curses_num = split.len();
            entries.push(split[1..].iter().map(|s| s.parse().unwrap()).collect());
        }
    }
    let mut result: u64 = 1;
    for e in 0..curses_num - 1 {
        let time = entries[0][e];
        let record = entries[1][e];
        let mut broker_records: u64 = 0;
        if quadra {
            let f = (time.pow(2) - 4 * record) as f64;
            let b1 = ((time as f64 + f.sqrt()) / 2.0).floor() as u64;
            let b2 = ((time as f64 - f.sqrt()) / 2.0).ceil() as u64;
            println!("b1={},b2={}", b1, b2);

            broker_records += b1 - b2 + 1;
        } else {
            for pressure in 0..time {
                let t = time - pressure;
                let distance = t * pressure;
                println!("Distance for ({}, {}) = {}", t, pressure, distance);
                if distance > record {
                    broker_records += 1;
                }
            }
        }
        result *= broker_records;
    }

    println!("Result = {}", result);
}

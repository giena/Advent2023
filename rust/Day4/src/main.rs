use std::fs::read_to_string;

fn main() {
    let mut result: u32 = 0;
    for line in read_to_string("./input").unwrap().lines() {
        let split1: Vec<&str> = line.split(['|', ':']).collect();

        //println!("Parse winners {}", split1[1]);
        let winners: Vec<u32> = split1[1]
            .trim()
            .split(' ')
            .map(|v| {
                if v.len() > 0 {
                    v.trim().parse().unwrap()
                } else {
                    0
                }
            })
            .filter(|v| v > &0)
            .collect();

        //println!("Parse numbers {}", split1[2]);
        let numbers: Vec<u32> = split1[2]
            .trim()
            .split(' ')
            .map(|v| {
                if v.len() > 0 {
                    v.trim().parse().unwrap()
                } else {
                    0
                }
            })
            .filter(|v| v > &0)
            .collect();

        let mut mywinners: Vec<u32> = vec![];

        for n in numbers {
            if winners.contains(&n) {
                mywinners.push(n);
            }
        }
        //println!("My winners {:?}", mywinners);
        if mywinners.len() > 0 {
            let size: u32 = mywinners.len().try_into().unwrap();
            let r = 2_u32.pow(size - 1);
            result += r;
            //println!("R = {} for {} ({})", r, size, result);
        }
    }
    println!("Result: {}", result);

    let mut cards: Vec<(Vec<u32>, Vec<u32>, usize, usize)> = vec![];
    for line in read_to_string("./input").unwrap().lines() {
        let split1: Vec<&str> = line.split(['|', ':']).collect();

        let winners: Vec<u32> = split1[1]
            .trim()
            .split(' ')
            .map(|v| {
                if v.len() > 0 {
                    v.trim().parse().unwrap()
                } else {
                    0
                }
            })
            .filter(|v| v > &0)
            .collect();

        let numbers: Vec<u32> = split1[2]
            .trim()
            .split(' ')
            .map(|v| {
                if v.len() > 0 {
                    v.trim().parse().unwrap()
                } else {
                    0
                }
            })
            .filter(|v| v > &0)
            .collect();

        let mut mywinners: Vec<u32> = vec![];
        for n in &numbers {
            if winners.contains(&n) {
                mywinners.push(*n);
            }
        }

        cards.push((winners, numbers, mywinners.len(), 1));
    }

    //println!("Final cards number: {:?}", cards);

    let mut result2: usize = 0;

    for i in 0..cards.len() {
        let card = cards[i].clone();
        let (_, _, wins, copies) = card;
        result2 += copies;
        let range = (i + 1)..(i + wins + 1);
        for j in range {
            if j < cards.len() {
                let mut new_card = &mut cards[j];
                let (_, _, _, copies2) = &mut new_card;
                *copies2 = *copies2 + copies;
            }
        }
    }

    println!("Result2 {}", result2);
}

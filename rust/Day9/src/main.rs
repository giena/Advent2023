use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("./input2").unwrap();
    let mut result: i64 = 0;
    for line in binding.lines() {
        println!("{}", line);
        let nums: Vec<i64> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        println!("Nums {:?}", nums);
        let mut solutions: Vec<Vec<i64>> = vec![nums];
        let mut i: usize = 0;
        loop {
            let vec: &Vec<i64> = solutions.get(i).unwrap();
            let mut r: Vec<i64> = vec![];
            for i in 0..vec.len() - 1 {
                r.push(vec.get(i + 1).unwrap() - vec.get(i).unwrap());
            }
            println!("r {:?}", r);
            if r.iter().all(|n| *n == 0) {
                break;
            };
            solutions.push(r);
            i = i + 1;
        }
        println!("Solutions {:?}", solutions);
        result = result
            + solutions
                .iter()
                .fold(0_i64, |acc, x| acc + x.last().unwrap());
    }
    println!("Result {}", result);
}

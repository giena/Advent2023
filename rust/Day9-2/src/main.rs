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
            let all_0 = r.iter().all(|n| *n == 0);
            solutions.push(r);
            i = i + 1;
            if all_0 {
                break;
            };
        }
        println!("Solutions {:?}", solutions);

        let len = solutions.len();

        for i in (0..solutions.len()).rev() {
            let mut v = solutions.get_mut(i).unwrap().clone();
            if i == len - 1 {
                println!("len");
                v.insert(0, 0);
            } else {
                println!(
                    "not len {} - {}",
                    v.first().unwrap(),
                    solutions.get(i + 1).unwrap().first().unwrap()
                );
                v.insert(
                    0,
                    v.first().unwrap() - solutions.get(i + 1).unwrap().first().unwrap(),
                );
            }
            println!("v={:?}", v);
            solutions[i] = v;
        }
        println!("Solutions2 {:?}", solutions);
        result = result
            + solutions
                .first()
                .into_iter()
                .fold(0_i64, |acc, x| acc + x.first().unwrap());
    }
    println!("Result {}", result);
}

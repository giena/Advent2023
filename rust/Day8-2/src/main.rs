use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::Cycle;
use std::str::Chars;
fn main() {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let binding = read_to_string("./input2").unwrap();
    let mut instructions: Option<Cycle<Chars<'_>>> = None;
    let mut a_terminated: Vec<&str> = vec![];

    for (cpt, line) in binding.lines().enumerate() {
        if cpt == 0 {
            instructions = Some(line.chars().cycle());
        } else if !line.trim().is_empty() {
            let split: Vec<&str> = line
                .split(['=', '(', ')', ','])
                .filter(|s| s.trim().len() > 0)
                .collect();
            let key = split[0].trim();
            if key.ends_with('A') {
                a_terminated.push(key);
            }
            nodes.insert(key, (split[1].trim(), split[2].trim()));
        }
    }

    println!("Nodes:{:?}", nodes);

    let steps: Vec<u64> = a_terminated
        .iter()
        .map(|p| {
            let mut steps: u64 = 0;
            let mut current_node = p;

            loop {
                let instruction = instructions.as_mut().unwrap().next();
                steps = steps + 1;
                current_node = match instruction {
                    Some('L') => &nodes.get(current_node).unwrap().0,
                    Some('R') => &nodes.get(current_node).unwrap().1,
                    Some(other) => panic!("Unknown instruction {}", other),
                    None => panic!("Missing instruction"),
                };
                if current_node.ends_with('Z') {
                    break;
                }
            }
            steps
        })
        .collect();

    println!("Steps {:?}", steps);
    let lcm = steps.iter().fold(1, |acc, e| lcm(acc, *e));
    println!("LCM {:?}", lcm);

    //travel(&nodes, instructions.as_mut().unwrap(), "AAA", 1);
}

fn gcd(a: u64, b: u64) -> u64 {
    let r = a % b;
    if r == 0 {
        b
    } else {
        gcd(b, r)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn travel(
    nodes: &HashMap<&str, (&str, &str)>,
    instructions: &mut Cycle<Chars<'_>>,
    node_name: &str,
    steps: u32,
) -> u32 {
    let tuple: Option<&(&str, &str)> = nodes.get(node_name);
    println!("tuple:{:?}", tuple);
    tuple.iter().for_each(|(node_l, node_r)| {
        let next_node = if instructions.next() == Some('L') {
            node_l
        } else {
            node_r
        };
        println!("Next:{}", next_node);
        if !next_node.ends_with('Z') {
            travel(nodes, instructions, *next_node, steps + 1);
        }
    });
    steps
}

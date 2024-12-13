use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::Cycle;
use std::str::Chars;

fn main() {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let binding = read_to_string("./input3").unwrap();
    let mut instructions: Option<Cycle<Chars<'_>>> = None;
    for (cpt, line) in binding.lines().enumerate() {
        if cpt == 0 {
            instructions = Some(line.chars().cycle());
        } else if !line.trim().is_empty() {
            let split: Vec<&str> = line
                .split(['=', '(', ')', ','])
                .filter(|s| s.trim().len() > 0)
                .collect();
            nodes.insert(split[0].trim(), (split[1].trim(), split[2].trim()));
        }
    }

    println!("Nodes:{:?}", nodes);
    let mut steps: u32 = 0;
    let mut next_node = "AAA";
    while next_node != "ZZZ" {
        let tuple: Option<&(&str, &str)> = nodes.get(next_node);
        next_node = match tuple {
            Some((node_l, node_r)) => {
                steps = steps + 1;
                if instructions.as_mut().unwrap().next() == Some('L') {
                    node_l
                } else {
                    node_r
                }
            }
            None => panic!("Unable to go to ZZZ"),
        }
    }
    println!("Step {}", steps);

    //travel(&nodes, instructions.as_mut().unwrap(), "AAA", 1);
}

fn travel(
    nodes: &HashMap<&str, (&str, &str)>,
    instructions: &mut Cycle<Chars<'_>>,
    node_name: &str,
    steps: u32,
) {
    let tuple: Option<&(&str, &str)> = nodes.get(node_name);
    println!("tuple:{:?}", tuple);
    tuple.iter().for_each(|(node_l, node_r)| {
        let next_node = if instructions.next() == Some('L') {
            node_l
        } else {
            node_r
        };
        println!("Next:{}", next_node);
        if *next_node != "ZZZ" {
            travel(nodes, instructions, *next_node, steps + 1);
        } else {
            println!("Step {}", steps);
        }
    })
}

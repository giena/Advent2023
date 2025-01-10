use std::collections::HashMap;
use std::fs::read_to_string;

fn hash(str: &str) -> u64 {
    let mut r: u64 = 0_u64;
    str.chars().for_each(|c| {
        r = r + c as u64;
        r = r * 17;
        if r > 256 {
            r = r % 256_u64;
        }
    });
    r
}

fn insert<'a>(boxes: &mut Vec<Vec<(&'a str, u32)>>, label: &'a str, distance: u32) {
    let index = hash(label) as usize;
    if let Some(i) = boxes[index].iter().position(|(l, _)| *l == label) {
        boxes[index].remove(i);
        boxes[index].insert(i, (label, distance));
    } else {
        boxes[index].push((label, distance));
    }
}

fn remove(boxes: &mut Vec<Vec<(&str, u32)>>, label: &str) {
    let index = hash(label) as usize;
    if let Some(i) = boxes[index].iter().position(|(l, _)| *l == label) {
        boxes[index].remove(i);
    }
}

fn total(boxes: &mut Vec<Vec<(&str, u32)>>) -> u64 {
    boxes
        .iter()
        .enumerate()
        .map(|(b, slots)| {
            slots
                .iter()
                .enumerate()
                .map(|(s, &(_, distance))| (b + 1) as u64 * (s + 1) as u64 * distance as u64)
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    let s = read_to_string("/home/giena/dev/work/advent/2023/rust/Day15/input").unwrap();
    let ops = s.split(',');
    /*let r: u64 = ops.map(|s| hash(s)).sum();
    println!("{}", r);*/

    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    ops.for_each(|op| {
        if op.ends_with("-") {
            remove(&mut boxes, op.strip_suffix("-").unwrap());
        } else if op.contains("=") {
            let parts: Vec<&str> = op.splitn(2, "=").collect();
            insert(&mut boxes, parts[0], parts[1].parse::<u32>().unwrap());
        }
    });
    println!("Boxes = {:?}", boxes);
    println!("Total = {}", total(&mut boxes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let hashes: Vec<u64> = s.split(',').map(|s| hash(s)).collect();
        assert!(hashes[0] == 30);
        assert!(hashes[1] == 253);
        assert!(hashes[2] == 97);
        assert!(hashes[3] == 47);
        assert!(hashes[4] == 14);
        assert!(hashes[5] == 180);
        assert!(hashes[6] == 9);
        assert!(hashes[7] == 197);
        assert!(hashes[8] == 48);
        assert!(hashes[9] == 214);
        assert!(hashes[10] == 231);
        assert!(hashes.iter().sum::<u64>() == 1320);
    }
}

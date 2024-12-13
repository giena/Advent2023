use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum Direction {
    Ground,
    Start,
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
}

trait Area {
    fn shoestring(&self) -> usize;
}

impl Area for Vec<(usize, usize)> {
    fn shoestring(&self) -> usize {
        (self.windows(2).fold(0, |acc, matrix| {
            acc + (matrix[0].0 * matrix[1].1) - (matrix[1].0 * matrix[0].1)
        }) / 2) as usize
    }
}

fn direction<'a>(pos: (usize, usize), map: &'a Vec<Vec<&Direction>>) -> Option<&'a Direction> {
    match map.get(pos.1) {
        Some(vec) => vec.get(pos.0).copied(),
        None => None,
    }
}

fn start_end<'a>(pos: (usize, usize), map: &'a Vec<Vec<&Direction>>) -> Vec<(usize, usize)> {
    let width = map.first().unwrap().len();

    let dir = direction(pos, map);
    match dir {
        Some(&Direction::Start) => {
            let mut ret: Vec<(usize, usize)> = vec![];
            if pos.1 > 0 {
                let dir = direction((pos.0, pos.1 - 1), &map);
                match dir {
                    Some(&Direction::Start) => panic!("Double start"),
                    Some(&Direction::NorthAndSouth) => ret.push((pos.0, pos.1 - 1)),
                    Some(&Direction::SouthAndEast) => ret.push((pos.0, pos.1 - 1)),
                    Some(&Direction::SouthAndWest) => ret.push((pos.0, pos.1 - 1)),
                    _ => (),
                }
            }

            if pos.1 < &map.len() - 1 {
                let dir = direction((pos.0, pos.1 + 1), &map);
                match dir {
                    Some(&Direction::Start) => panic!("Double start"),
                    Some(&Direction::NorthAndSouth) => ret.push((pos.0, pos.1 + 1)),
                    Some(&Direction::NorthAndEast) => ret.push((pos.0, pos.1 + 1)),
                    Some(&Direction::NorthAndWest) => ret.push((pos.0, pos.1 + 1)),
                    _ => (),
                }
            }

            if pos.0 > 0 {
                let dir = direction((pos.0 - 1, pos.1), &map);
                match dir {
                    Some(&Direction::Start) => panic!("Double start"),
                    Some(&Direction::EastAndWest) => ret.push((pos.0 - 1, pos.1)),
                    Some(&Direction::NorthAndEast) => ret.push((pos.0 - 1, pos.1)),
                    Some(&Direction::SouthAndEast) => ret.push((pos.0 - 1, pos.1)),
                    _ => (),
                }
            }

            if pos.0 < width - 1 {
                let dir = direction((pos.0 + 1, pos.1), &map);
                match dir {
                    Some(&Direction::Start) => panic!("Double start"),
                    Some(&Direction::EastAndWest) => ret.push((pos.0 + 1, pos.1)),
                    Some(&Direction::NorthAndWest) => ret.push((pos.0 + 1, pos.1)),
                    Some(&Direction::SouthAndWest) => ret.push((pos.0 + 1, pos.1)),
                    _ => (),
                }
            }

            assert!(ret.len() == 2);
            ret
        }
        _ => panic!("Not a start pos {:?} = {:?}", pos, dir),
    }
}

fn walk(
    old_pos: (usize, usize),
    current_pos: (usize, usize),
    map: &Vec<Vec<&Direction>>,
) -> (usize, usize) {
    let dir = direction(current_pos, map);
    let go_to_south = old_pos.1 < current_pos.1;
    let go_to_north = old_pos.1 > current_pos.1;
    let go_to_east = old_pos.0 < current_pos.0;
    let go_to_west = old_pos.0 > current_pos.0;
    match dir {
        None => panic!("None is met"),
        Some(&Direction::Start) => panic!("Start is met"),
        Some(&Direction::Ground) => panic!("Ground is met"),
        Some(&Direction::NorthAndSouth) => {
            if go_to_south {
                (current_pos.0, current_pos.1 + 1)
            } else if go_to_north {
                (current_pos.0, current_pos.1 - 1)
            } else {
                panic!("{:?}, {:?}, {:?}", dir, old_pos, current_pos);
            }
        }
        Some(&Direction::EastAndWest) => {
            if go_to_east {
                (current_pos.0 + 1, current_pos.1)
            } else if go_to_west {
                (current_pos.0 - 1, current_pos.1)
            } else {
                panic!("{:?}, {:?}, {:?}", dir, old_pos, current_pos);
            }
        }
        Some(&Direction::NorthAndEast) => {
            if go_to_south {
                (current_pos.0 + 1, current_pos.1)
            } else if go_to_west {
                (current_pos.0, current_pos.1 - 1)
            } else {
                panic!("{:?}, {:?}, {:?}", dir, old_pos, current_pos);
            }
        }
        Some(&Direction::NorthAndWest) => {
            if go_to_south {
                (current_pos.0 - 1, current_pos.1)
            } else if go_to_east {
                (current_pos.0, current_pos.1 - 1)
            } else {
                panic!("{:?}, {:?}, {:?}", dir, old_pos, current_pos);
            }
        }
        Some(&Direction::SouthAndEast) => {
            if go_to_north {
                (current_pos.0 + 1, current_pos.1)
            } else if go_to_west {
                (current_pos.0, current_pos.1 + 1)
            } else {
                panic!("{:?}, {:?}, {:?}", dir, old_pos, current_pos);
            }
        }
        Some(&Direction::SouthAndWest) => {
            if go_to_north {
                (current_pos.0 - 1, current_pos.1)
            } else if go_to_east {
                (current_pos.0, current_pos.1 + 1)
            } else {
                panic!("{:?}, {:?}, {:?}", dir, old_pos, current_pos);
            }
        }
    }
}

fn visit(
    init_pos: (usize, usize),
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    map: &Vec<Vec<&Direction>>,
) {
    let mut old_pos: (usize, usize) = init_pos;
    let mut current_pos: (usize, usize) = start_pos;
    println!("Curr Pos {:?}", current_pos);

    let mut steps = 2_u64;

    loop {
        let temp_pos = current_pos;
        current_pos = walk(old_pos, current_pos, &map);
        println!("Curr Pos {:?}", current_pos);
        old_pos = temp_pos;
        if current_pos == end_pos {
            break;
        }
        steps = steps + 1;
    }

    println!("Steps {}", steps);
}

fn main() {
    let directions: HashMap<char, Direction> = HashMap::from([
        ('.', Direction::Ground),
        ('S', Direction::Start),
        ('|', Direction::NorthAndSouth),
        ('-', Direction::EastAndWest),
        ('L', Direction::NorthAndEast),
        ('J', Direction::NorthAndWest),
        ('7', Direction::SouthAndWest),
        ('F', Direction::SouthAndEast),
    ]);

    let mut S: (usize, usize) = (0, 0);

    let mut map: Vec<Vec<&Direction>> = vec![];

    let binding = read_to_string("./input4").unwrap();

    for (y, line) in binding.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        match chars.iter().position(|c| *c == 'S') {
            Some(pos) => S = (pos, y),
            None => (),
        }
        map.push(chars.iter().map(|c| directions.get(&c).unwrap()).collect());
    }

    println!("Start {:?}", S);
    println!("Map {:?}", map);

    let start_end = start_end(S, &map);

    println!("Start/End {:?}", start_end);

    let start = *start_end.first().unwrap();
    let end = *start_end.last().unwrap();

    visit(S, start, end, &map)
}

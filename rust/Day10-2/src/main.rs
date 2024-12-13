use std::cmp;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::usize;

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
    fn point_in_polygon(&self, point: (i32, i32)) -> bool;
}

impl Area for Vec<(i32, i32)> {
    fn point_in_polygon(&self, point: (i32, i32)) -> bool {
        let num_vertices = self.len();
        let mut inside = false;
        let mut p1 = self.get(0).unwrap();
        let mut p2: &(i32, i32);
        for i in 1..=self.len() {
            p2 = self.get(i % num_vertices).unwrap();
            if point.1 > cmp::min(p1.1, p2.1) {
                if point.1 <= cmp::max(p1.1, p2.1) {
                    if point.0 <= cmp::max(p1.0, p2.0) {
                        let x_intersection: f64 = (point.1 as f64 - p1.1 as f64)
                            * (p2.0 as f64 - p1.0 as f64)
                            / (p2.1 as f64 - p1.1 as f64)
                            + p1.0 as f64;
                        if p1.0 == p2.0 || point.0 as f64 <= x_intersection {
                            // Flip the inside flag
                            inside = !inside;
                        }
                    }
                }
            }
            p1 = p2;
        }
        inside
    }
}

fn direction<'a>(pos: (i32, i32), map: &'a Vec<Vec<&Direction>>) -> Option<&'a Direction> {
    match map.get(pos.1 as usize) {
        Some(vec) => vec.get(pos.0 as usize).copied(),
        None => None,
    }
}

fn start_end<'a>(pos: (i32, i32), map: &'a Vec<Vec<&Direction>>) -> Vec<(i32, i32)> {
    let width: i32 = map.first().unwrap().len().try_into().unwrap();

    let dir = direction(pos, map);
    match dir {
        Some(&Direction::Start) => {
            let mut ret: Vec<(i32, i32)> = vec![];
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

            if pos.1 < map.len().try_into().unwrap() {
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

fn walk(old_pos: (i32, i32), current_pos: (i32, i32), map: &Vec<Vec<&Direction>>) -> (i32, i32) {
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
    init_pos: (i32, i32),
    start_pos: (i32, i32),
    end_pos: (i32, i32),
    map: &Vec<Vec<&Direction>>,
) {
    let mut old_pos: (i32, i32) = init_pos;
    let mut current_pos: (i32, i32) = start_pos;
    println!("Curr Pos {:?}", current_pos);

    let mut visited: Vec<(i32, i32)> = vec![start_pos];

    loop {
        let temp_pos = current_pos;
        current_pos = walk(old_pos, current_pos, &map);
        println!("Curr Pos {:?}", current_pos);
        old_pos = temp_pos;
        visited.push(current_pos);
        if current_pos == end_pos {
            break;
        }
    }

    println!("Steps {}", visited.len() / 2 + 1);

    let mut points_inside: u32 = 0;
    for (y, vec_p) in map.iter().enumerate() {
        for x in 0..vec_p.len() {
            if !visited.contains(&(x as i32, y as i32)) {
                if visited.point_in_polygon((x as i32, y as i32)) {
                    points_inside = points_inside + 1;
                }
            }
        }
    }

    println!("Points inside {}", points_inside);
}

fn main() {
    let square = vec![(0, 0), (4, 0), (4, 4), (0, 4)];
    println!("In Square: {}", square.point_in_polygon((2, 2)));

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

    let mut S: (i32, i32) = (0, 0);

    let mut map: Vec<Vec<&Direction>> = vec![];

    let binding = read_to_string("./input4").unwrap();

    for (y, line) in binding.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        match chars.iter().position(|c| *c == 'S') {
            Some(pos) => S = (pos as i32, y as i32),
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

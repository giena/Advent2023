use std::{borrow::BorrowMut, fs::read_to_string};

struct Game {
    tiles: Vec<char>,
    width: usize,
    len: usize,
}

impl<'a> Game {
    pub fn new(lines: Vec<&str>, width: usize) -> Self {
        let tiles: Vec<char> = lines.iter().flat_map(|str| str.chars()).collect();
        Game {
            tiles: tiles,
            width,
            len: lines.len(),
        }
    }

    fn column(&self, index: usize) -> Vec<char> {
        assert!(index < self.width);
        let mut out: Vec<char> = vec![];
        for i in 1..=self.len {
            out.push(self.tiles[index + self.width * (i - 1)]);
        }
        out
    }

    pub fn tilt_north(&self) {
        let mut result = 0_u64;
        for s in self.tiles.chunks(self.width) {
            println!("{}", s.iter().collect::<String>());
        }
        let mut new_tiles = self.tiles.clone();
        for c in 0..self.width {
            let column: Vec<char> = self.column(c);
            //println!("In: {:?}", column);
            let mut new_column: Vec<char> = vec![];
            let groups: Vec<&[char]> = column.split(|c| *c == '#').collect();
            for g in 0..groups.len() {
                let group = groups[g];
                let solids = group.iter().filter(|c| **c == 'O').count();
                let dots = group.len() - solids;
                if solids > 0 {
                    new_column.append(&mut vec!['O'; solids]);
                }
                if dots > 0 {
                    new_column.append(&mut vec!['.'; dots]);
                }
                if g < groups.len() - 1 {
                    new_column.push('#');
                }
            }
            //println!("New Column: {:?}", new_column);
            for i in 1..=new_column.len() {
                let tile = new_column[i - 1];
                if tile == 'O' {
                    result = result + (new_column.len() as u64 + 1 - i as u64);
                }
                new_tiles[c + self.width * (i - 1)] = new_column[i - 1];
            }
        }
        println!("Result = {}", result);
    }
}

fn main() {
    let binding = read_to_string("/home/giena/dev/work/advent/2023/rust/Day14/input2").unwrap();
    //let mut games: Vec<Game> = vec![];
    let mut lines: Vec<&str> = vec![];
    let mut width: usize = 0;
    for line in binding.lines() {
        lines.push(line);
        width = line.len();
    }
    let game = Game::new(lines, width);
    game.tilt_north();
}

use std::fs::read_to_string;

struct Game<'a> {
    lines: Vec<&'a str>,
    width: Option<usize>,
    part2: bool,
}

impl<'a> Game<'a> {
    pub fn new(lines: Vec<&'a str>, part2: bool) -> Self {
        Game {
            lines: lines.clone(),
            width: lines.first().map(|line| line.len()),
            part2,
        }
    }

    fn column(&self, index: usize) -> Vec<char> {
        let mut out: Vec<char> = vec![];
        for s in &self.lines {
            let chars: Vec<char> = s.chars().collect();
            out.push(chars[index]);
        }
        out
    }

    fn line(&self, index: usize) -> Vec<char> {
        self.lines[index].chars().collect()
    }

    pub fn check_horizontal(&self) -> usize {
        let width = self.width.unwrap();
        let mut ret: usize = 0;
        for c in 0..(width - 1) {
            let mut ok = true;
            for dc in 0..width {
                if c >= dc {
                    let left = c - dc;
                    let right = c + 1 + dc;
                    if left < right && right < width {
                        let cleft = self.column(left);
                        let cright = self.column(right);
                        if cleft != cright {
                            if !self.part2 {
                                println!("Test H {} {} ({},{}) Not OK", left, right, c, dc);
                                ok = false;
                            } else {
                                let diff =
                                    cleft.iter().zip(cright).filter(|(a, b)| **a != *b).count();
                                if diff > 1 {
                                    println!(
                                        "Test H {} {} ({},{}) Not OK (part2)",
                                        left, right, c, dc
                                    );
                                    ok = false;
                                } else {
                                    println!(
                                        "Test H {} {} ({},{}) OKKKK (part2)",
                                        left, right, c, dc
                                    );
                                }
                            }
                        } else {
                            println!("Test {} {} ({},{}) OKKKK", left, right, c, dc);
                        }
                    }
                }
            }
            if ok {
                ret = ret + c + 1;
            }
        }
        ret
    }

    pub fn check_vertical(&self) -> usize {
        let mut ret: usize = 0;
        for r in 0..(self.lines.len() - 1) {
            let mut ok = true;
            for dr in 0..self.lines.len() {
                if r >= dr {
                    let up = r - dr;
                    let down = r + 1 + dr;
                    if up < down && down < self.lines.len() {
                        let rup = self.line(up);
                        let rdown = self.line(down);
                        if rup != rdown {
                            if !self.part2 {
                                println!("Test V {} {} ({},{}) Not OK", up, down, r, dr);
                                ok = false;
                            } else {
                                let diff = rup.iter().zip(rdown).filter(|(a, b)| **a != *b).count();
                                if diff > 1 {
                                    println!(
                                        "Test V {} {} ({},{}) Not OK (part2)",
                                        up, down, r, dr
                                    );
                                    ok = false;
                                } else {
                                    println!("Test V {} {} ({},{}) OKKKK (part2)", up, down, r, dr);
                                }
                            }
                        }
                    }
                }
            }
            if ok {
                ret = ret + 100 * (r + 1);
            }
        }
        ret
    }
}

fn main() {
    let mut total: u64 = 0_u64;
    let binding = read_to_string("/home/giena/dev/work/advent/2023/rust/Day13/input").unwrap();
    //let mut games: Vec<Game> = vec![];
    let mut lines: Vec<&str> = vec![];
    println!("***********************************************************");
    println!("***********************************************************");
    let part2 = false;
    for line in binding.lines() {
        if line.trim().is_empty() {
            let game = Game::new(lines.clone(), part2);
            let r: usize = game.check_horizontal();
            println!("Horizontal = {:?}", r);
            total = total + r as u64;
            let r = game.check_vertical();
            println!("Vertical = {:?}", r);
            total = total + r as u64;
            println!("***********************************************************");
            println!("***********************************************************");
            lines.clear();
        } else {
            lines.push(line);
        }
    }
    let game = Game::new(lines.clone(), part2);
    let r = game.check_horizontal();
    println!("Horizontal = {:?}", r);
    total = total + r as u64;
    let r = game.check_vertical();
    println!("Vertical = {:?}", r);
    total = total + r as u64;
    println!("TOTAL = {:?}", total);
}

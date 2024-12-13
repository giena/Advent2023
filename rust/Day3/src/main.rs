use std::fmt;
use std::fs::read_to_string;

struct Symbol {
    x: usize,
    y: usize,
    value: char,
}

impl Symbol {
    fn new(x: usize, y: usize, value: char) -> Self {
        Symbol {
            x: x,
            y: y,
            value: value,
        }
    }
}

struct Number {
    x: usize,
    y: usize,
    len: usize,
    value: u32,
}

impl Number {
    fn new(x: usize, y: usize, len: usize, value: u32) -> Self {
        Number {
            x: x,
            y: y,
            len: len,
            value: value,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.len, self.value)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.value)
    }
}

fn main() {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (y, line) in read_to_string("./input").unwrap().lines().enumerate() {
        let mut number: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if !number.is_empty() {
                        let len = number.len();
                        let s: String = number.into_iter().collect();
                        numbers.push(Number::new(x - len, y, len, s.parse().unwrap()));
                        number = vec![];
                    }
                }
                o => {
                    if o.is_digit(10) {
                        number.push(o);
                        if x == line.len() - 1 {
                            let len = number.len();
                            let s: String = number.into_iter().collect();
                            numbers.push(Number::new(x - len, y, len, s.parse().unwrap()));
                            number = vec![];
                        }
                    } else {
                        symbols.push(Symbol::new(x, y, c));
                        if !number.is_empty() {
                            let len = number.len();
                            let s: String = number.into_iter().collect();
                            numbers.push(Number::new(x - len, y, len, s.parse().unwrap()));
                            number = vec![];
                        }
                    }
                }
            }
        }
    }
    println!("Numbers: {}", numbers.len());
    println!("Symbols: {}", symbols.len());

    let mut sum = 0;

    for number in &numbers {
        for symbol in &symbols {
            if symbol.y == number.y || symbol.y.abs_diff(number.y) == 1 {
                if symbol.x >= number.x.checked_sub(1).unwrap_or(0)
                    && symbol.x <= number.x + number.len
                {
                    println!("Symbol {} is adjacent to number {}", symbol, number);
                    sum += number.value;
                }
            }
        }
    }

    println!("Sum: {}", sum);

    let mut sum = 0;

    for symbol in &symbols {
        if symbol.value == '*' {
            let mut adjacents: Vec<&Number> = vec![];
            for number in &numbers {
                if symbol.y == number.y || symbol.y.abs_diff(number.y) == 1 {
                    if symbol.x >= number.x.checked_sub(1).unwrap_or(0)
                        && symbol.x <= number.x + number.len
                    {
                        println!("Symbol {} is adjacent to number {}", symbol, number);
                        adjacents.push(&number);
                    }
                }
            }
            if adjacents.len() == 2 {
                let mut mul = 1;
                for adjacent in adjacents {
                    mul *= adjacent.value;
                }
                sum += mul;
            }
        }
    }
    println!("Sum: {}", sum);
}

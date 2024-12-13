use std::{fs::read_to_string, num::ParseIntError};

fn analyze(line: &str) -> Result<u32, ParseIntError> {
    let letters = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut digits_pos: Vec<(usize, usize)> = vec![];
    let mut letters_pos: Vec<(usize, usize)> = vec![];

    for (x, c) in line.chars().enumerate() {
        let pos = digits.iter().position(|p| *p == c);
        if pos.is_some() {
            digits_pos.push((x, pos.unwrap()));
        }
    }

    for x in 0..line.len() {
        let substr = &line[x..];
        for l in 0..letters.len() {
            if substr.starts_with(letters[l]) {
                letters_pos.push((x, l));
            }
        }
    }

    let first_digits = digits_pos.first();
    let first_letters = letters_pos.first();
    let last_digits = digits_pos.last();
    let last_letters = letters_pos.last();

    let first = if first_letters.is_none() {
        first_digits
    } else {
        if first_digits.is_none() {
            first_letters
        } else {
            if first_letters.unwrap().0 < first_digits.unwrap().0 {
                first_letters
            } else {
                first_digits
            }
        }
    };

    let last = if last_letters.is_none() {
        last_digits
    } else {
        if last_digits.is_none() {
            last_letters
        } else {
            if last_letters.unwrap().0 > last_digits.unwrap().0 {
                last_letters
            } else {
                last_digits
            }
        }
    };

    let mut str = String::new();
    str.push(digits[first.or(last).unwrap().1]);
    str.push(digits[last.or(first).unwrap().1]);

    let result = str.parse();
    println!(
        "Treating {}\t\t = {:?} ({:?},{:?},{:?},{:?})",
        line, result, first_digits, first_letters, last_digits, last_letters
    );
    result
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("input").unwrap().lines() {
        let r = analyze(line);
        sum = sum + r.unwrap();
    }
    println!("{}", sum);
}

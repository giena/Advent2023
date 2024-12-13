use find_all::FindAll;
use std::fs::read_to_string;
use std::usize;

fn get_column<'a>(map: &'a Vec<Vec<char>>, col_idx: usize) -> Vec<&char> {
    let mut ret: Vec<&char> = vec![];
    for line in map {
        ret.push(line.get(col_idx).unwrap());
    }
    ret
}

fn manhattan(
    a: &(usize, usize),
    b: &(usize, usize),
    coeff: usize,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
) -> usize {
    let (col_min, col_max) = (a.0.min(b.0), a.0.max(b.0));
    let (row_min, row_max) = (a.1.min(b.1), a.1.max(b.1));

    let delta_col = col_max - col_min;
    let delta_row = row_max - row_min;

    let col_exp = empty_columns
        .iter()
        .filter(|&col| *col >= col_min && *col <= col_max)
        .count();
    let row_exp = empty_rows
        .iter()
        .filter(|&row| *row >= row_min && *row <= row_max)
        .count();

    delta_col + col_exp * (coeff - 1) + delta_row + row_exp * (coeff - 1)
}

fn sum_manhattans(
    galaxies: &Vec<(usize, usize)>,
    coeff: usize,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
) -> u64 {
    let len = galaxies.len();
    (0..len - 1)
        .flat_map(|i| (i + 1..len).map(move |j| (galaxies[i], galaxies[j])))
        .map(|(first, other)| manhattan(&first, &other, coeff, empty_rows, empty_columns) as u64)
        .sum()
}

fn main() {
    let mut map: Vec<Vec<char>> = vec![];

    let binding = read_to_string("./input2").unwrap();

    let mut width = 0;

    let mut empty_rows: Vec<usize> = vec![];

    for (y, line) in binding.lines().enumerate() {
        width = line.len();
        map.push(line.chars().collect());
        if !line.contains('#') {
            empty_rows.push(y);
        }
    }

    println!("Empty rows {:?}", empty_rows);

    let mut empty_columns: Vec<usize> = vec![];

    for x in 0..width {
        let column = get_column(&map, x);
        if !column.contains(&&'#') {
            empty_columns.push(x);
        }
    }

    println!("Empty columns {:?}", empty_columns);

    let mut galaxies: Vec<(usize, usize)> = vec![];

    for (y, line) in map.iter().enumerate() {
        match line.iter().find_all(|c| **c == '#') {
            Some(pos) => {
                for x in pos {
                    galaxies.push((x, y));
                }
            }
            None => (),
        }
    }

    println!("Galaxies {:?}", galaxies);

    println!(
        "d = {}",
        sum_manhattans(&galaxies, 1000000, &empty_rows, &empty_columns)
    );
}

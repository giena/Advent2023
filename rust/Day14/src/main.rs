use std::collections::HashMap;
use std::fs::read_to_string;

struct Game {
    tiles: Vec<char>,
    width: usize,
    len: usize,
}

impl<'a> Game {
    pub fn new(tiles: Vec<char>, width: usize, len: usize) -> Self {
        Game {
            tiles: tiles,
            width,
            len,
        }
    }
}

fn column(game: &Game, index: usize) -> Vec<char> {
    assert!(index < game.width);
    let mut out: Vec<char> = vec![];
    for i in 1..=game.len {
        out.push(game.tiles[index + game.width * (i - 1)]);
    }
    out
}

fn row(game: &Game, index: usize) -> Vec<char> {
    assert!(index < game.len);
    let mut out: Vec<char> = vec![];
    for i in 0..game.width {
        out.push(game.tiles[i + game.width * index]);
    }
    out
}

fn tilt_east(game: &Game) -> Game {
    let mut new_tiles = game.tiles.clone();
    for r in 0..game.len {
        let row: Vec<char> = row(game, r);
        let mut new_row: Vec<char> = vec![];
        let groups: Vec<&[char]> = row.split(|c| *c == '#').collect();
        for g in 0..groups.len() {
            let group = groups[g];
            let solids = group.iter().filter(|c| **c == 'O').count();
            let dots = group.len() - solids;
            if dots > 0 {
                new_row.append(&mut vec!['.'; dots]);
            }
            if solids > 0 {
                new_row.append(&mut vec!['O'; solids]);
            }
            if g < groups.len() - 1 {
                new_row.push('#');
            }
        }
        for i in 0..new_row.len() {
            new_tiles[i + game.width * r] = new_row[i];
        }
    }
    Game::new(new_tiles, game.width, game.len)
}

fn tilt_west(game: &Game) -> Game {
    let mut new_tiles = game.tiles.clone();
    for r in 0..game.len {
        let row: Vec<char> = row(game, r);
        let mut new_row: Vec<char> = vec![];
        let groups: Vec<&[char]> = row.split(|c| *c == '#').collect();
        for g in 0..groups.len() {
            let group = groups[g];
            let solids = group.iter().filter(|c| **c == 'O').count();
            let dots = group.len() - solids;
            if solids > 0 {
                new_row.append(&mut vec!['O'; solids]);
            }
            if dots > 0 {
                new_row.append(&mut vec!['.'; dots]);
            }
            if g < groups.len() - 1 {
                new_row.push('#');
            }
        }
        for i in 0..new_row.len() {
            new_tiles[i + game.width * r] = new_row[i];
        }
    }
    Game::new(new_tiles, game.width, game.len)
}

fn tilt_north(game: &Game) -> Game {
    let mut new_tiles = game.tiles.clone();
    for c in 0..game.width {
        let column: Vec<char> = column(game, c);
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
        for i in 1..=new_column.len() {
            new_tiles[c + game.width * (i - 1)] = new_column[i - 1];
        }
    }
    Game::new(new_tiles, game.width, game.len)
}

fn tilt_south(game: &Game) -> Game {
    let mut new_tiles = game.tiles.clone();
    for c in 0..game.width {
        let column: Vec<char> = column(game, c);
        let mut new_column: Vec<char> = vec![];
        let groups: Vec<&[char]> = column.split(|c| *c == '#').collect();
        for g in 0..groups.len() {
            let group = groups[g];
            let solids = group.iter().filter(|c| **c == 'O').count();
            let dots = group.len() - solids;
            if dots > 0 {
                new_column.append(&mut vec!['.'; dots]);
            }
            if solids > 0 {
                new_column.append(&mut vec!['O'; solids]);
            }
            if g < groups.len() - 1 {
                new_column.push('#');
            }
        }
        for i in 1..=new_column.len() {
            new_tiles[c + game.width * (i - 1)] = new_column[i - 1];
        }
    }
    Game::new(new_tiles, game.width, game.len)
}

fn cycle(game: &Game) -> Game {
    tilt_east(&tilt_south(&tilt_west(&tilt_north(game))))
}

fn total(game: &Game) -> usize {
    let chunks = game.tiles.chunks(game.width);
    chunks
        .enumerate()
        .map(|(y, row)| {
            let nb_round = row.iter().filter(|&&r| r == 'O').count();
            nb_round * (game.len - y)
        })
        .sum()
}

fn main() {
    let binding = read_to_string("/home/giena/dev/work/advent/2023/rust/Day14/input2").unwrap();
    let mut lines: Vec<&str> = vec![];
    let mut width: usize = 0;
    for line in binding.lines() {
        lines.push(line);
        width = line.len();
    }
    let tiles: Vec<char> = lines.iter().flat_map(|str| str.chars()).collect();
    let mut game = Game::new(tiles, width, lines.len());
    game.tiles
        .chunks(game.width)
        .for_each(|s| println!("{:?}", s));

    let first_game = tilt_north(&game);
    println!("First = {}", total(&first_game));

    println!("*****************************");

    let mut results: HashMap<Vec<char>, usize> = HashMap::new();

    let mut i = 0;
    let cycle_length: usize = loop {
        if let Some(old_id) = results.insert(game.tiles.clone(), i) {
            break i - old_id;
        }
        game = cycle(&game);
        i += 1;
    };

    println!("Repetition: {} {}", cycle_length, i);
    /*for i in results.len() - 1 - cycle_length * 3..results.len() - 1 {
        println!("Result {} = {}", i, results[i]);
    }*/

    //i cycles are done.
    //It keeps to do (1000000000 - i) to be done.
    //But this reapeats each cycle_length, so compute the remaining with modulo.
    let remaining = (1000000000 - i) % cycle_length;
    println!("Rem: {}", remaining);
    for _ in 0..remaining {
        game = cycle(&game);
    }

    let score = total(&game);
    println!("Total = {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let initial: &str = "O....#....
                    O.OO#....#
                    .....##...
                    OO.#O....O
                    .O.....O#.
                    O.#..O.#.#
                    ..O..#O..O
                    .......O..
                    #....###..
                    #OO..#....";

        let after_cycle1 = ".....#....
                                    ....#...O#
                                    ...OO##...
                                    .OO#......
                                    .....OOO#.
                                    .O#...O#.#
                                    ....O#....
                                    ......OOOO
                                    #...O###..
                                    #..OO#....";

        let after_cycle2 = ".....#....
                                ....#...O#
                                .....##...
                                ..O#......
                                .....OOO#.
                                .O#...O#.#
                                ....O#...O
                                .......OOO
                                #..OO###..
                                #.OOO#...O";

        let after_cycle3 = ".....#....
                                ....#...O#
                                .....##...
                                ..O#......
                                .....OOO#.
                                .O#...O#.#
                                ....O#...O
                                .......OOO
                                #...O###.O
                                #.OOO#...O";

        let lines: Vec<&str> = initial.split_ascii_whitespace().collect();
        let tiles: Vec<char> = lines.iter().flat_map(|str| str.chars()).collect();
        let width = lines.first().unwrap().len();
        let mut game = Game::new(tiles, width, lines.len());

        game = cycle(&game);
        let lines: Vec<&str> = after_cycle1.split_ascii_whitespace().collect();
        let tiles: Vec<char> = lines.iter().flat_map(|str| str.chars()).collect();
        assert!(game.tiles == tiles);

        game = cycle(&game);
        let lines: Vec<&str> = after_cycle2.split_ascii_whitespace().collect();
        let tiles: Vec<char> = lines.iter().flat_map(|str| str.chars()).collect();
        assert!(game.tiles == tiles);

        game = cycle(&game);
        let lines: Vec<&str> = after_cycle3.split_ascii_whitespace().collect();
        let tiles: Vec<char> = lines.iter().flat_map(|str| str.chars()).collect();
        assert!(game.tiles == tiles);
    }
}

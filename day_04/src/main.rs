use clap::Parser;
use itertools::{FoldWhile, Itertools};
use std::{env, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "example.txt")]
    input: String,
}

fn search_for_word(grid: &[Vec<char>], row: usize, column: usize, word: &[char; 4]) -> usize {
    let mut occurances = 0;

    let total_rows = grid.len() as i32;
    let total_columns = grid[0].len() as i32;

    // NOTE: (Stefan) We want to early exit if the first character doesn't match.
    if grid[row][column] != word[0] {
        return 0;
    }

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (x, y) in directions {
        let mut curr_x = row as i32 + x;
        let mut curr_y = column as i32 + y;

        let full_match = word
            .iter()
            .skip(1) // NOTE: (Stefan) Skip the first character as it has already been checked!
            .fold_while(true, |_, character| {
                if curr_x >= total_rows || curr_x < 0 || curr_y >= total_columns || curr_y < 0 {
                    return FoldWhile::Done(false);
                }

                if grid[curr_x as usize][curr_y as usize] != *character {
                    return FoldWhile::Done(false);
                };

                curr_x += x;
                curr_y += y;

                FoldWhile::Continue(true)
            })
            .into_inner();

        if full_match {
            occurances += 1;
        }
    }

    occurances
}

fn main() {
    let args = Args::parse();
    let crate_name = clap::crate_name!();
    let cli_inputpath = path::Path::new(&args.input);

    let input_data = aocutils::read_input_file_by_line(cli_inputpath, crate_name);

    let grid = input_data
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total_columns = grid[0].len();
    let target_word = &['X', 'M', 'A', 'S'];

    let count = grid.iter().flatten().enumerate().fold(0, |acc, (idx, _)| {
        let column = idx / total_columns;
        let row = idx % total_columns;

        acc + search_for_word(&grid, row, column, target_word)
    });

    println!("Total occurances of 'XMAS' (in any direction): {count}");
}

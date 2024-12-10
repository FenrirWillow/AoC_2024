use clap::Parser;
use itertools::{FoldWhile, Itertools};
use std::{env, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "example.txt")]
    input: String,
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

    let total_rows = grid.len();
    let total_columns = grid[0].len();
    let word = &['X', 'M', 'A', 'S'];

    let count = grid
        .iter()
        .flatten()
        .enumerate()
        .map(|(idx, _)| {
            let column = idx / total_columns;
            let row = idx % total_columns;

            (row, column)
        })
        .fold(0, |total_count, (row, column)| {
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

            total_count
                + directions.iter().fold(
                    0,
                    |total_occurrences_in_all_directions, (row_offset, column_offset)| {
                        let occurrences = word
                            .iter()
                            .enumerate()
                            .map(|(idx, character)| {
                                let next_row = row as i32 + idx as i32 * row_offset;
                                let next_column = column as i32 + idx as i32 * column_offset;

                                (character, next_row, next_column)
                            })
                            .fold_while(0, |_, (character, next_row, next_column)| {
                                if !(0..total_rows as i32).contains(&next_row)
                                    || !(0..total_columns as i32).contains(&next_column)
                                {
                                    return FoldWhile::Done(0);
                                }

                                if grid[next_row as usize][next_column as usize] != *character {
                                    return FoldWhile::Done(0);
                                };

                                FoldWhile::Continue(1)
                            })
                            .into_inner();

                        total_occurrences_in_all_directions + occurrences
                    },
                )
        });

    println!("Total occurrences of 'XMAS' (in any direction): {count}");
}

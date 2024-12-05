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

    let safe_level_count = input_data
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|level_values| {
            let final_difference = level_values
                .iter()
                .tuple_windows()
                .fold_while::<i32, _>(0, |previous_difference, (n1, n2)| {
                    let diff = n1 - n2;
                    let abs_diff = i32::abs(diff);

                    if !(1..=3).contains(&abs_diff) {
                        // NOTE: (Stefan) This level is considered "unsafe", because
                        // the value of the difference is not within the specified range
                        return FoldWhile::Done(0);
                    }

                    if previous_difference != 0 && previous_difference.signum() != diff.signum() {
                        // NOTE: (Stefan) This level is considered "unsafe", because
                        // the difference changed "direction" from last window
                        return FoldWhile::Done(0);
                    }

                    FoldWhile::Continue(diff)
                })
                .into_inner();

            final_difference != 0
        })
        .filter(|is_safe| *is_safe)
        .count();

    println!("The total number of safe levels is: {}", safe_level_count)
}

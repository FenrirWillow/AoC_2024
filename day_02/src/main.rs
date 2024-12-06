use clap::Parser;
use itertools::{FoldWhile, Itertools};
use std::{collections::HashSet, env, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "example.txt")]
    input: String,
}

fn is_level_safe(level_values: &[i32]) -> bool {
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
}

fn main() {
    let args = Args::parse();
    let crate_name = clap::crate_name!();
    let cli_inputpath = path::Path::new(&args.input);

    let input_data = aocutils::read_input_file_by_line(cli_inputpath, crate_name);

    let reports = input_data
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_by_default = HashSet::<usize>::new();

    // #region Part One

    let safe_level_count = reports
        .iter()
        .map(|level_values| is_level_safe(level_values))
        .enumerate()
        .map(|(idx, is_safe)| {
            // NOTE: (Stefan) This may be cleaner with a partition map.
            // We can separate the levels into "safe_by_default" and "potentially_safe"
            // We can then use the "potentially_safe" levels as the source for the iterator
            // in the second part.
            if is_safe {
                safe_by_default.insert(idx);
            }

            is_safe
        })
        .filter(|is_safe| *is_safe)
        .count();

    println!("The total number of safe levels is: {}", safe_level_count);

    // #endregion

    // #region Part Two

    let safe_with_dampen_count = reports
        .iter()
        .enumerate()
        .filter(|(idx, _)| !safe_by_default.contains(idx))
        .map(|(_, level_values)| {
            level_values
                .iter()
                .combinations(level_values.len() - 1)
                // NOTE: (Stefan) The following map is needed because the combinations iterator creates a really weird iterator type. The
                // transform makes it slightly more manageable.
                // REFACTOR: (Stefan) I wonder if there is a cleaner way to deal with this?
                .map(|subset_level_values| {
                    subset_level_values.iter().map(|v| **v).collect::<Vec<_>>()
                })
                .map(|subset_level_values| is_level_safe(&subset_level_values))
                .any(|is_subset_safe| is_subset_safe)
        })
        .filter(|any_subset_safe| *any_subset_safe)
        .count();

    println!(
        "The total number of safe levels (with dampener on) is: {}",
        safe_level_count + safe_with_dampen_count
    );

    // #endregion
}
